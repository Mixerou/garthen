use amqp::{Channel, ExchangeKind, Queue};
use amqp::options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use amqp::types::FieldTable;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use lapin::Connection;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmqpPayload {
    DispatchData {
        device_id: i64,
    },
    RequestData {
        device_id: Option<i64>,
        greenhouse_id: Option<i64>,
    },
    Ping,
}

#[derive(Clone, Debug)]
pub struct AmqpClient<'a> {
    pub connection: &'a Connection,
    pub channel: Channel,
}

impl<'a> AmqpClient<'a> {
    pub fn init() -> Self {
        let runtime = Runtime::new().unwrap();
        let connection = amqp::get_connection();
        let channel = runtime.block_on(async move {
            connection.create_channel().await.expect("Failed to create AMQP channel")
        });
        let client = AmqpClient { connection, channel };

        runtime.block_on(async move {
            // Exchanges
            AmqpClient::declare_exchange(&client.channel, "data", ExchangeKind::Topic).await;

            // Queues
            AmqpClient::declare_queue(&client.channel, "request-data").await;
            AmqpClient::declare_queue(&client.channel, "dispatch-data").await;

            // Queue bindings
            AmqpClient::bind_queue(
                &client.channel,
                "request-data",
                "data",
                "data.request",
            ).await;
            AmqpClient::bind_queue(
                &client.channel,
                "dispatch-data",
                "data",
                "data.create",
            ).await;

            client
        })
    }

    async fn declare_exchange(channel: &Channel, name: &str, kind: ExchangeKind) {
        channel.exchange_declare(
            name,
            kind,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        ).await.unwrap_or_else(|_| panic!("Failed to declare {name} exchange"))
    }

    async fn declare_queue(channel: &Channel, name: &str) -> Queue {
        channel.queue_declare(
            name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        ).await.unwrap_or_else(|_| panic!("Failed to declare {name} queue"))
    }

    async fn bind_queue(channel: &Channel, name: &str, exchange: &str, routing_key: &str) {
        channel.queue_bind(
            name,
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await.unwrap_or_else(|_| panic!(
            "Failed to bind {name} queue to {exchange} exchange with {routing_key} routing key",
        ))
    }
}
