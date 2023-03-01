# AMQP Library

AMQP is a client that can connect to various message brokers,
such as `RabbitMQ`, using the `amqp` protocol.

## Usage

Add to project

```toml
[dependencies]
amqp = { path = "@/libs/amqp" }
```

This library use [Lapin](https://crates.io/crates/lapin)
that follows the [`AMQP 0.9.1` specifications](https://www.rabbitmq.com/resources/specs/amqp0-9-1.pdf),
targeting especially `RabbitMQ`.
To find out how to interact with message broker,
refer to the [Lapin documentation](https://docs.rs/lapin).

## Environment Variables

| Variable   | Default Value | Description                                                                                        |
|------------|:-------------:|----------------------------------------------------------------------------------------------------|
| `AMQP_URL` |       -       | URL to your message broker server in the format `amqp://{username}:{password}@{domain/ip}:{port}`. |
