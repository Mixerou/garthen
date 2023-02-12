# Snowflake Generator Library

Rust version of the Twitter Snowflake algorithm.

## Usage

Add to project

```toml
[dependencies]
snowflake-generator = { path = "@/libs/snowflake-generator" }
```

Write some Rust

```rust
extern crate snowflake_generator as snowflake;

fn main() {
    snowflake::init();
    
    println!("Your ID is: {}", snowflake::generate())
}
```

## Environment Variables

| Variable               | Default Value | Description                                                |
|------------------------|:-------------:|------------------------------------------------------------|
| `SNOWFLAKE_MACHINE_ID` |       -       | The ID of the machine on which the application is running. |
| `SNOWFLAKE_NODE_ID`    |       -       | The ID of the node on which the application is running.    |
