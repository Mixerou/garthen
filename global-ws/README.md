# Garthen Global WS

Global WebSocket module of the Garthen Project

## Setup

```bash
# Serve with hot reload
$ cargo install cargo-watch
$ cargo watch -c -x run

# Build for production (Only Tier 1 targets recommended)
$ cargo build --release --target=<arch><sub>-<vendor>-<sys>-<abi>
```

## Environment Variables

[`DATABASE_URL`]: ../libs/db/README.md#environment-variables
[`AMQP_URL`]: ../libs/amqp/README.md#environment-variables
[`SNOWFLAKE_MACHINE_ID`]: ../libs/snowflake-generator/README.md#environment-variables
[`SNOWFLAKE_NODE_ID`]: ../libs/snowflake-generator/README.md#environment-variables

| Variable                 | Default Value | Description                                                                                                                   |
|--------------------------|:-------------:|-------------------------------------------------------------------------------------------------------------------------------|
| `RUST_LOG`               |       -       | `env_logger` output controller. Module declarations take comma separated entries formatted like `path::to::module=log_level`. |
| `GLOBAL_WS_IP`           |  `127.0.0.1`  | IP on which the Global WS will run.                                                                                           |
| `GLOBAL_WS_PORT`         |    `9000`     | The port that the Global WS will listen to.                                                                                   |
| `GLOBAL_WS_PATH`         | Empty string  | Domain path to Global WS. Do not add `/` at the end.                                                                          |
| [`DATABASE_URL`]         |       -       | URL to your postgres database in the format `postgres://{username}:{password}@{domain/ip}/garthen`.                           |
| [`AMQP_URL`]             |       -       | URL to your message broker server in the format `amqp://{username}:{password}@{domain/ip}:{port}`.                            |
| [`SNOWFLAKE_MACHINE_ID`] |       -       | The ID of the machine on which the application is running.                                                                    |
| [`SNOWFLAKE_NODE_ID`]    |       -       | The ID of the node on which the application is running.                                                                       |
