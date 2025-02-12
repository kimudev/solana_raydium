use anyhow::{anyhow, Error};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};
use substreams_solana_utils::transaction::{get_signature, get_signers};
use raydium_amm_substream;
use raydium_amm_substream::raydium_amm::constants::RAYDIUM_AMM_PROGRAM_ID;
use raydium_amm_substream::pb::raydium_amm::raydium_amm_event;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde_json::json;
use std::time::Duration;

mod instruction;
use instruction::{get_indexed_instructions, IndexedInstruction, IndexedInstructions};

#[substreams::handlers::map]
fn block_kafka_output(block: Block) -> Result<(), Error> {
    let producer = create_kafka_producer()?;

    for (index, transaction) in block.transactions.iter().enumerate() {
        if let Some(json_event) = parse_raydium_swap(transaction, index as u32, block.slot)? {
            send_to_kafka(&producer, "raydium_amm_events", json_event)?;
        }
    }

    Ok(())
}

fn parse_raydium_swap(transaction: &ConfirmedTransaction, transaction_index: u32, slot: u64) -> Result<Option<String>, Error> {
    if transaction.meta.as_ref().unwrap().err.is_some() {
        return Ok(None);
    }

    let instructions = get_indexed_instructions(transaction)?;
    for instruction in instructions.flattened().iter() {
        if instruction.program_id() != RAYDIUM_AMM_PROGRAM_ID {
            continue;
        }
        if let Some(raydium_amm_event::Event::Swap(swap)) = raydium_amm_substream::parse_instruction(&instruction.instruction, &get_signers(transaction)).ok().flatten() {
            let swap_event = json!({
                "slot": slot,
                "transaction_index": transaction_index,
                "instruction_index": instruction.index,
                "amm": swap.amm,
                "user": swap.user,
                "amount_in": swap.amount_in,
                "amount_out": swap.amount_out,
                "mint_in": swap.mint_in,
                "mint_out": swap.mint_out,
                "direction": swap.direction,
                "pool_pc_amount": swap.pool_pc_amount.unwrap_or(0),
                "pool_coin_amount": swap.pool_coin_amount.unwrap_or(0),
                "pc_mint": swap.pc_mint,
                "coin_mint": swap.coin_mint,
                "user_pre_balance_in": swap.user_pre_balance_in.unwrap_or(0),
                "user_pre_balance_out": swap.user_pre_balance_out.unwrap_or(0)
            });
            return Ok(Some(swap_event.to_string()));
        }
    }
    Ok(None)
}

fn create_kafka_producer() -> Result<FutureProducer, Error> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .map_err(|e| anyhow!("Failed to create Kafka producer: {}", e))?;
    Ok(producer)
}

fn send_to_kafka(producer: &FutureProducer, topic: &str, message: serde_json::Value) -> Result<(), Error> {
    let record = FutureRecord::to(topic).key("key").payload(&message.to_string());
    producer.send(record, Duration::from_secs(0)).map_err(|e| anyhow!("Failed to send message to Kafka: {}", e))?;
    Ok(())
}
