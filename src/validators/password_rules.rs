use validator::ValidationError;

pub struct ValidationContext;

pub fn password_rules(value: &str) -> Result<(), ValidationError> {
    if value.len() < 8 {
        return Err(ValidationError::new(
            "Password must be at least 8 characters long",
        ));
    }

    if !value.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }

    if !value.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }

    if !value.chars().any(|c| c.is_digit(10)) {
        return Err(ValidationError::new(
            "Password must contain at least one number",
        ));
    }

    if !value
        .chars()
        .any(|c| "!@#$%^&*()-_=+[]{}|;:'\",.<>?/".contains(c))
    {
        return Err(ValidationError::new(
            "Password must contain at least one special character",
        ));
    }

    Ok(())
}
