use std::str::from_utf8;
use std::thread;

use actix::{Actor, Context, ContextFutureSpawner, Handler, WeakAddr, WrapFuture};
use actix_broker::BrokerSubscribe;
use amqp::{BasicProperties, Channel, ExchangeKind, Queue};
use amqp::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use amqp::types::FieldTable;
use futures::executor::block_on;
use futures::StreamExt;

use crate::messages::{AmqpPayload, AmqpPublisherMessage, DispatchAmqpMessage, InitAmqpConsumersMessage};
use crate::server::Socket;

#[derive(Debug)]
pub struct AmqpClient {
    pub channel: Channel,
}

impl AmqpClient {
    pub async fn new() -> Self {
        let connection = amqp::get_connection();
        let channel = connection.create_channel()
            .await.expect("Failed to create AMQP channel");

        AmqpClient {
            channel,
        }
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

    fn start_consumer(socket: WeakAddr<Socket>, name: &str, queue: &str) {
        let name = name.to_string();
        let queue = queue.to_string();

        thread::spawn(move || block_on(async move {
            info!("Starting AMQP {name} consumer thread");

            let connection = amqp::get_connection();
            let channel = connection.create_channel().await
                .unwrap_or_else(|_| panic!("Failed to create AMQP channel in {name} consumer"));

            let mut consumer = channel.basic_consume(
                queue.as_str(),
                format!("global-ws-{name}").as_str(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await.unwrap_or_else(|_| panic!("Failed to create AMQP {name} consumer"));

            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.unwrap();

                if let Ok(payload) = serde_json::from_str::<AmqpPayload>(
                    from_utf8(delivery.data.as_slice()).unwrap_or("")
                ) {
                    socket.upgrade().unwrap().do_send(DispatchAmqpMessage { payload });
                }

                delivery.ack(BasicAckOptions::default())
                    .await.unwrap_or_else(|_| panic!("Failed to ACK in {name} consumer"));
            }
        }));
    }
}

impl Actor for AmqpClient {
    type Context = Context<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        let channel = self.channel.clone();

        block_on(async move {
            // Exchanges
            AmqpClient::declare_exchange(&channel, "data", ExchangeKind::Topic).await;

            // Queues
            AmqpClient::declare_queue(&channel, "request-data").await;
            AmqpClient::declare_queue(&channel, "dispatch-data").await;

            // Queue bindings
            AmqpClient::bind_queue(
                &channel,
                "request-data",
                "data",
                "data.request",
            ).await;
            AmqpClient::bind_queue(
                &channel,
                "dispatch-data",
                "data",
                "data.create",
            ).await;
        });

        self.subscribe_system_async::<InitAmqpConsumersMessage>(context);
        self.subscribe_system_async::<AmqpPublisherMessage>(context);
    }
}

impl Handler<InitAmqpConsumersMessage> for AmqpClient {
    type Result = ();

    fn handle(&mut self, message: InitAmqpConsumersMessage, _: &mut Self::Context) -> Self::Result {
        AmqpClient::start_consumer(message.0, "data-dispatcher", "dispatch-data");
    }
}

impl<'a> Handler<AmqpPublisherMessage<'a>> for AmqpClient {
    type Result = ();

    fn handle(&mut self, message: AmqpPublisherMessage, context: &mut Self::Context) -> Self::Result {
        let channel = self.channel.clone();
        let exchange = message.exchange.unwrap_or("").to_string();
        let routing_key = message.routing_key.unwrap_or("").to_string();

        async move {
            if let Ok(payload) = serde_json::to_string(&message.payload) {
                channel.basic_publish(
                    exchange.as_str(),
                    routing_key.as_str(),
                    BasicPublishOptions::default(),
                    payload.as_bytes(),
                    BasicProperties::default(),
                ).await.unwrap().await.unwrap();
            }
        }.into_actor(self).spawn(context);
    }
}
