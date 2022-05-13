#[cfg(test)]
pub mod test_utils {
    use fake::{faker::internet::en::SafeEmail, Fake, StringFaker};
    use once_cell::sync::Lazy;

    use crate::telemetry;

    const ASCII: &str =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@";
    const ALLOWED_USERNAME_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

    static TRACING: Lazy<()> = Lazy::new(|| {
        let subscriber = telemetry::get_subscriber("test".into(), "debug".into());
        telemetry::init_subscriber(subscriber);
    });

    pub fn lazy_init_subscriber() {
        Lazy::force(&TRACING);
    }

    pub fn random_ascii_string(len: std::ops::Range<usize>) -> String {
        StringFaker::with(Vec::from(ASCII), len).fake()
    }

    pub fn random_valid_username() -> String {
        StringFaker::with(Vec::from(ALLOWED_USERNAME_CHARS), 3..24).fake()
    }

    pub fn random_valid_email() -> String {
        SafeEmail().fake()
    }
}
