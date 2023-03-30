use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

pub struct Telemetry;

impl Telemetry {
    pub fn initialize() {
        let stdout = tracing_subscriber::fmt::layer().pretty();
        Registry::default().with(stdout).init();
    }
}
