# Database Library

DB is the most productive way to interact with database.

## Setup

Run all commands in the `libs/db` directory

```bash
# For Linux (Before doing so, install one of these `libpq-dev`, `postgresql-libs`, etc)
$ cargo install diesel_cli --no-default-features --features postgres

# For MacOS
$ brew install libpq
$ cargo install diesel_cli --no-default-features --features postgres

# For Windows (PowerShell)
$ choco install postgres
$ setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\15\lib" # Or any other path to your PostgreSQL lib
$ cargo install diesel_cli --no-default-features --features postgres
```

## Usage

Add to project

```toml
[dependencies]
db = { path = "@/libs/db" }
diesel = { version = "2.0.3", default-features = false }
r2d2 = { version = "0.8.10", default-features = false }
```

This library use [diesel](https://diesel.rs/) to interact with database
because of its safe and composable abstractions over queries.
To find out how to interact with database, refer to the [Diesel documentation](https://diesel.rs/guides/).

## Environment Variables

| Variable       | Default Value | Description                                                                                         |
|----------------|:-------------:|-----------------------------------------------------------------------------------------------------|
| `DATABASE_URL` |       -       | URL to your postgres database in the format `postgres://{username}:{password}@{domain/ip}/garthen`. |
