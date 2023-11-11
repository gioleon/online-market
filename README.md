# online-market

In this project we're using Rust programming language, Postgres and axum framework.

To run this project first create a **.env** file with the following attributes:

**DATABASE_URL=postgres://user:password@localhost:9999/database**

**RUST_LOG=your_log_level for example, debug**

then, go to the route online-market-data and execute

**sqlx migration run --database-url=postgres://user:password@localhost:9999/database**

and then, in the route online-market-axum exceute

**cargo run**