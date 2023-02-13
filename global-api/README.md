# Garthen Global API

Global API module of the Garthen Project

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
[`SNOWFLAKE_MACHINE_ID`]: ../libs/snowflake-generator/README.md#environment-variables
[`SNOWFLAKE_NODE_ID`]: ../libs/snowflake-generator/README.md#environment-variables

| Variable                 | Default Value | Description                                                                                                                   |
|--------------------------|:-------------:|-------------------------------------------------------------------------------------------------------------------------------|
| `RUST_LOG`               |       -       | `env_logger` output controller. Module declarations take comma separated entries formatted like `path::to::module=log_level`. |
| `GLOBAL_API_IP`          |  `127.0.0.1`  | IP on which the Global API will run.                                                                                          |
| `GLOBAL_API_PORT`        |    `5000`     | The port that the Global API will listen to.                                                                                  |
| `GLOBAL_API_PATH`        | Empty string  | Domain path to Global API. Do not add `/` at the end.                                                                         |
| [`DATABASE_URL`]         |       -       | URL to your postgres database in the format `postgres://{username}:{password}@{domain/ip}/garthen`.                           |
| [`SNOWFLAKE_MACHINE_ID`] |       -       | The ID of the machine on which the application is running.                                                                    |
| [`SNOWFLAKE_NODE_ID`]    |       -       | The ID of the node on which the application is running.                                                                       |
