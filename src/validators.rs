use validator::ValidationError;

pub fn is_lowercase_alphanumeric(s: &str) -> Result<(), ValidationError> {
    s.chars()
        .all(|c| (c.is_alphanumeric() && c.is_lowercase()) || c == '_')
        .then_some(())
        .ok_or(ValidationError::new(
            "Only lowercase and alphabetic are allowed",
        ))
}
