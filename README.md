# microservice-rust-cat
This is a repo for a microservice in rust that returns a random category from a list of categories. This microservice is a example of how to create a microservice in rust for my Integration course at the University of the Americas.

# Dependencies

- [actix-web](https://actix.rs/docs/web/)
- [sqlx](https://github.com/launchbadge/sqlx)
- [dotenv](https://github.com/dotenv-rs/dotenv)
- [mysql](https://github.com/mysql-rs/mysql-async)

# How to run

1. Clone the repo
2. Run `cargo run`

# How to test

1. Run `cargo test` for running all tests
2. Run `cargo test -- --test-threads=1` for running all tests in a single thread
3. Run `cargo test test_categorize_endpoint` for running a specific test called `test_categorize_endpoint`
4. Run `cargo test test_categorize_object` for running a specific test called `test_categorize_object`

# How to build

1. Run `cargo build --release`
2. Run `cargo build --release --target x86_64-pc-windows-gnu`


