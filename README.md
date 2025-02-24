The repo contains a number of components referenced in this blog post https://paragraph.xyz/@ayc/recover-safe-wallet-signers

forge/ has the custom contracts, tests and when you build it you get the bytecode to put into `onchain`.

src/onchain requires `ETH_RPC` to be set so it can walk through all WazirX wallet transactions. 
src/offchain works on data extracted from bigquery and allowed me to cover every Safe wallet transaction (ever).

For example to start decoding Bybit transactions;
```
cargo run --bin onchain -- --safe-wallet-address 0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4 --start-block 11820678 --end-block 21895239
```

To start decoding WazirX transactions;
```
cargo run --bin onchain -- --safe-wallet-address 0x27fD43BABfbe83a81d14665b1a6fB8030A60C9b4 --start-block 16132578 --end-block 20331566
```


The BigQuery looks like this;
```
WITH matched_transactions AS (
    SELECT transaction_hash
    FROM `bigquery-public-data.goog_blockchain_ethereum_mainnet_us.logs` 
    WHERE "0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e" IN UNNEST(topics)
    --AND block_number > 16132577
)
SELECT t.transaction_hash, t.input
FROM `bigquery-public-data.goog_blockchain_ethereum_mainnet_us.transactions` t
WHERE t.transaction_hash IN (SELECT transaction_hash FROM matched_transactions)
and LEFT(t.input, 34) = '0x6a761202000000000000000000000000'
```
