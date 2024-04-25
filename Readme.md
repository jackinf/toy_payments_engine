# Toy Payments Engine

This is a simple Rust application that simulates a transaction system. It reads transactions from a CSV file, processes them, and outputs the results to another CSV file.

## Installation

Make sure you have Rust and Cargo installed on your machine. If not, you can download and install them from [the official Rust website](https://www.rust-lang.org/tools/install).

## Quickstart

You can run the program with the following command:
```bash
cargo run -- transactions.csv > accounts.csv
```

## Testing

You can run all unit and integration tests with the following command:
```bash
cargo test
```

## Benchmarks

The benchmarks measure the performance of the transaction processing system. You can run them with the following command:
```bash
cargo bench
```

Before running the benchmarks, you might need to make sure you have git lfs installed and have pulled the large sample file with the following commands:
```bash
git lfs install
git lfs fetch
git lfs checkout
```
(Although this step is optional if file in HEAD is less than 100MB.)

## Assumptions

After reading the requirements, I made the following assumptions:
- Negative balances are not allowed when doing withdrawals.
- Deposits and withdrawals can be disputed
- If a withdrawal is disputed, the available balance is affected only when a chargeback occurs. If the withdrawal for account A is disputed, then it's like disputing a deposit for account B
- Transactions with an id that has already been seen are ignored.
- If the file parsing fails at any stage (invalid row format), the program will exit
- If the balance becomes negative after a disputed withdrawal, then that's okay for my toy application

I might have missed some cases, when it comes to negative balances, but I think the above assumptions are reasonable.