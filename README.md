# solana-kafka-indexer
Индексатор блокчейна Solana с использованием Substreams и Kafka.

## Поддерживаемые программы
- Raydium AMM (только события обмена `swap`)

Если у вас есть предложения по поддержке других программ, откройте issue!

## Использование
1. Используйте тег v0.1.5: `git clone https://github.com/0xpapercut/solana-indexer.git --branch v0.1.5`
2. Установите зависимости: `cargo install substreams-cli`
3. Настройте переменную окружения `STREAMINGFAST_KEY`.
4. Запустите `. ./token.sh`, чтобы установить переменную `SUBSTREAMS_API_TOKEN`.
5. Запустите индексатор с помощью `make stream START=<слот>`.

## Примечания
- Индексатор передает **только события обмена (`swap`) Raydium AMM** в Kafka.
- Kafka должен быть запущен на `localhost:9092`.
- Обработанные данные публикуются в топик Kafka `raydium_amm_events`.
- Убедитесь, что у вас есть **Kafka-консьюмер**, который считывает данные из топика.

Если возникнут вопросы или ошибки, откройте issue!
