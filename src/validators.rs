use validator::ValidationError;

pub fn is_lowercase_alphabetic(s: &str) -> Result<(), ValidationError> {
    s.chars()
        .all(|c| c.is_alphabetic() && c.is_lowercase())
        .then_some(())
        .ok_or(ValidationError::new(
            "Only lowercase and alphabetic are allowed",
        ))
}
