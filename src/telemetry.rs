use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub struct Telemetry;

impl Telemetry {
    pub fn initialize() {
        let stdout = tracing_subscriber::fmt::layer().pretty();
        let filter = EnvFilter::try_from_default_env().unwrap();
        Registry::default().with(stdout).with(filter).init();
        LogTracer::init().expect("Failed to set logger!");
    }
}
