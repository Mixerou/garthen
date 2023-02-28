# Garthen Data Worker

Data Worker module of the Garthen Project

## Setup

```bash
# Serve with hot reload
$ cargo install cargo-watch
$ cargo watch -c -x run

# Build for production (Only Tier 1 targets recommended)
$ cargo build --release --target=<arch><sub>-<vendor>-<sys>-<abi>
```

## Environment Variables

| Variable   | Default Value | Description                                                                                                                   |
|------------|:-------------:|-------------------------------------------------------------------------------------------------------------------------------|
| `RUST_LOG` |       -       | `env_logger` output controller. Module declarations take comma separated entries formatted like `path::to::module=log_level`. |
