use super::models::RegisterUser;

pub fn validate_register_user(user_data: &RegisterUser) -> Result<(), String> {
    let mut errors = Vec::<String>::new(); // Vector to store error messages

    if user_data.first_name.is_empty() {
        errors.push("First Name is required".into());
    }

    if user_data.last_name.is_empty() {
        errors.push("Last Name is required".into());
    }

    if user_data.username.is_empty() {
        errors.push("Username is required".into());
    }
    if user_data.email.is_empty() {
        errors.push("Email is required".into());
    }
    if user_data.password.is_empty() {
        errors.push("Password is required".into());
    }

    // Validate phone number (basic format check)
    if user_data.phone.is_empty() {
        errors.push("Phone number is required".into());
    } else {
        let phone_regex = regex::Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap(); // Example regex for international phone numbers
        if !phone_regex.is_match(&user_data.phone) {
            errors.push("Invalid phone number format".into());
        }
    }

    // Validate title and image (if required)
    if user_data.title.is_empty() {
        errors.push("Title is required".into());
    }
    if user_data.image.is_empty() {
        errors.push("Image is required".into());
    }

    // Example of email format validation (basic regex)
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    if !email_regex.is_match(&user_data.email) {
        errors.push("Invalid email format".into());
    }

    // Add more validation as needed (e.g., password strength)
    // ...

    if !errors.is_empty() {
        return Err(errors.join(",")); // Return all errors as a single string
    }

    Ok(()) // All validations passed
}
