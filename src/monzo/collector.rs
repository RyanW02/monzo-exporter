use super::Client;
use std::collections::HashMap;
use tokio::time::{delay_for, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::monzo::Metric;
use crate::monzo::metric;

pub struct Collector {
    client: Arc<Client>,
    pub metrics: RwLock<HashMap<Box<str>, Metric>>,
}

const STAT_BALANCE: &str = "monzo_balance";
const STAT_BALANCE_FLEXIBLE_SAVINGS: &str = "monzo_balance_including_flexible_savings";

impl Collector {
    pub fn new(client: Arc<Client>) -> Collector {
        Collector {
            client,
            metrics: RwLock::new(HashMap::new()),
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                match self.client.get_balance().await {
                    Ok(response) => {
                        let balance = Metric::Float {
                            value: (response.balance as f64) / 100.0,
                            metric_type: metric::METRIC_GAUGE,
                            help: Box::from("Current account balance")
                        };
                        let balance_flexible_savings = Metric::Float {
                            value: (response.balance_including_flexible_savings as f64) / 100.0,
                            metric_type: metric::METRIC_GAUGE,
                            help: Box::from("Balance including pots")
                        };

                        let metrics = &mut *self.metrics.write().await;
                        metrics.insert(Box::from(STAT_BALANCE), balance);
                        metrics.insert(Box::from(STAT_BALANCE_FLEXIBLE_SAVINGS), balance_flexible_savings);
                    }

                    Err(e) => {
                        eprintln!("Error retrieving balance: {:?}", e);
                    }
                }

                // TODO: Make tweakable
                delay_for(Duration::from_secs(15)).await;
            }
        });
    }
}