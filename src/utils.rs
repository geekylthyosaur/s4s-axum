#[cfg(test)]
pub mod test_utils {
    use once_cell::sync::Lazy;

    use crate::telemetry;

    static TRACING: Lazy<()> = Lazy::new(|| {
        let subscriber = telemetry::get_subscriber("test".into(), "debug".into());
        telemetry::init_subscriber(subscriber);
    });

    pub fn lazy_init_subscriber() {
        Lazy::force(&TRACING);
    }
}