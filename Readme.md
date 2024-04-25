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
- I do not allow negative balances when doing withdrawals
- I can dispute deposits
- I can dispute withdrawals
- If I dispute the withdrawal for account A, then it's like disputing a deposit for account B
- if I dispute the withdrawal, then the available balance is affected only when chargeback occurs
- If I deposit/withdraw using the transaction id that has already been seen, then the transaction is ignored
- If the file parsing fails at any stage (invalid row format), the program will exit
- If the balance becomes negative after a disputed withdrawal, then that's okay for my toy application

I might have missed some cases, when it comes to negative balances, but I think the above assumptions are reasonable.