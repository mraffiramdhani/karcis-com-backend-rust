use crate::domain::models::auth::{LoginPayload, RegisterPayload};

pub struct AuthValidator;

impl AuthValidator {
    pub fn validate_register_payload(payload: &RegisterPayload) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Validation logic here
        if payload.first_name.is_empty() {
            errors.push("First Name is required".into());
        }
        if payload.last_name.is_empty() {
            errors.push("Last Name is required".into());
        }
        if payload.username.is_empty() {
            errors.push("Username is required".into());
        }
        if payload.email.is_empty() {
            errors.push("Email is required".into());
        }
        let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
        if !email_regex.is_match(&payload.email) {
            errors.push("Invalid email format".into());
        }
        if payload.password.is_empty() {
            errors.push("Password is required".into());
        }
        if payload.phone.is_empty() {
            errors.push("Phone number is required".into());
        } else {
            let phone_regex = regex::Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap(); // Example regex for international phone numbers
            if !phone_regex.is_match(&payload.phone) {
                errors.push("Invalid phone number format".into());
            }
        }
        if payload.title.is_empty() {
            errors.push("Title is required".into());
        }
        if payload.image.is_empty() {
            errors.push("Image is required".into());
        }

        if !errors.is_empty() {
            return Err(ValidationError::Multiple(errors));
        }

        Ok(())
    }

    pub fn validate_login_payload(payload: &LoginPayload) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if payload.username.is_empty() {
            errors.push("Username is required".into());
        }
        if payload.password.is_empty() {
            errors.push("Password is required".into());
        }

        if !errors.is_empty() {
            return Err(ValidationError::Multiple(errors));
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Validation failed: {0}")]
    Single(String),
    #[error("Multiple validation errors: {}", .0.join(", "))]
    Multiple(Vec<String>),
}
