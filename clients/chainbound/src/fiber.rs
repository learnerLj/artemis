use anyhow::Result;
use async_trait::async_trait;
use fiber::Client;
use futures::StreamExt;

// Use the actual types returned by fiber streams
use alloy_consensus::{Block, TxEnvelope};

use artemis_core::types::{Collector, CollectorStream};

const FIBER_DEFAULT_URL: &str = "beta.fiberapi.io:8080";

/// Possible events emitted by the Fiber collector.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum Event {
    Transaction(TxEnvelope),
    ExecutionPayload(Block<TxEnvelope>),
}

/// Fiber collector stream type, used to specify which stream to subscribe to.
pub enum StreamType {
    /// Subscribe to new pending transactions as seen by the Fiber network.
    Transactions,
    /// Subscribe to new execution payloads (blocks with full transaction data).
    ExecutionPayloads,
}

/// A Fiber collector that subscribes to the specified stream type.
pub struct FiberCollector {
    /// The Fiber-rs client
    client: Client,
    /// The Fiber API key
    api_key: String,
    /// The type of stream to subscribe to
    ty: StreamType,
}

impl FiberCollector {
    /// Initialize a new Fiber collector.
    ///
    /// ## Arguments
    /// - `api_key`: The Fiber API key to use
    /// - `ty`: The type of stream to subscribe to
    pub async fn new(api_key: String, ty: StreamType) -> Self {
        let client = Client::connect(FIBER_DEFAULT_URL, api_key.clone())
            .await
            .expect("failed to connect to Fiber");

        Self {
            client,
            api_key,
            ty,
        }
    }

    /// Optionally set the Fiber endpoint, overriding the default
    pub async fn set_fiber_endpoint(&mut self, endpoint: impl Into<String>) {
        self.client = Client::connect(endpoint, self.api_key.clone())
            .await
            .expect("failed to connect to Fiber");
    }

    /// Get the event stream for the specified stream type.
    pub async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        match self.ty {
            StreamType::Transactions => {
                let stream = self.client.subscribe_new_transactions(None).await;
                let stream = stream.map(|tx| Event::Transaction(tx.into_inner()));
                Ok(Box::pin(stream))
            }
            StreamType::ExecutionPayloads => {
                let stream = self.client.subscribe_new_execution_payloads().await;
                let stream = stream.map(Event::ExecutionPayload);
                Ok(Box::pin(stream))
            }
        }
    }
}

#[async_trait]
impl Collector<Event> for FiberCollector {
    async fn get_event_stream<'a>(&'a self) -> Result<CollectorStream<'a, Event>> {
        self.get_event_stream().await
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use artemis_core::engine::Engine;
    use ethers::types::Action;

    use crate::Event;
    use crate::FiberCollector;
    use crate::StreamType;

    #[tokio::test]
    async fn test_fiber_collector_txs() -> Result<()> {
        if let Ok(api_key) = std::env::var("FIBER_TEST_KEY") {
            let fiber_collector = FiberCollector::new(api_key, StreamType::Transactions).await;

            let mut engine: Engine<Event, Action> = Engine::default();
            engine.add_collector(Box::new(fiber_collector));

            if let Ok(mut set) = engine.run().await {
                while let Some(res) = set.join_next().await {
                    println!("res: {:?}", res);
                }
            }
        } else {
            println!("Skipping Fiber test, no API key found in FIBER_TEST_KEY env var");
        }

        Ok(())
    }
}
