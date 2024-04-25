# Toy transaction

## Quickstart

Running the program
```bash
cargo run -- transactions.csv > results.csv
```

Run all unit/integration tests
```bash
cargo test
```

## Benchmarks

Make sure that you have git lfs installed
```bash
git lfs install
```

Pull large sample file
```bash
git lfs fetch
git lfs checkout
```

Running benchmarks
```bash
cargo bench
```

## Assumptions

After reading the requirements, I made the following assumptions:
- I can dispute both deposits and withdrawals (and nothing else)
- If I dispute the withdrawal, then it's like disputing a deposit but with a reversed sign
- If I deposit/withdraw using the transaction id that has already been seen, then the transaction is ignored
- When chargeback occurs, I just remove held amount, and don't recover the available amount. I think this is a quick/dirty alternative to creating a reverse transaction
- If the file parsing fails at any stage (invalid row format), the program will exit