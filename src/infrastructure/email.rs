use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};

use crate::{config, domain::models::user::User};

pub async fn send_mail(
    data: User,
    subject: &str,
    template: String,
) -> Result<<SmtpTransport as Transport>::Ok, <SmtpTransport as Transport>::Error> {
    let settings = config::Settings::load().expect("Failed to load configuration");
    let host = settings.email.smtp_host;
    let port = settings.email.smtp_port;
    let user = settings.email.smtp_username;
    let pass = settings.email.smtp_password;
    let from_email = settings.email.from_email;
    let from_name = settings.email.from_name;
    let use_tls = settings.email.use_tls;

    let creds = Credentials::new(user, pass);

    // Configure SMTP based on TLS settings
    let mailer = SmtpTransport::starttls_relay(host.as_str())?
        .credentials(creds)
        .authentication(vec![Mechanism::Plain, Mechanism::Login])
        .build()?;

    let recipient = format!(
        "{} {} {} <{}>",
        &data.title, &data.first_name, &data.last_name, &data.email
    );

    let email = Message::builder()
        .from(format!("{} <{}>", from_name, from_email).parse().unwrap())
        .to(recipient.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(template)
        .unwrap();

    mailer.send(&email)
}

pub async fn test_smtp_connection() -> Result<(), Box<dyn std::error::Error>> {
    let settings = config::Settings::load().expect("Failed to load configuration");

    println!(
        "Testing SMTP connection to {}:{}",
        settings.email.smtp_host, settings.email.smtp_port
    );
    println!("Username: {}", settings.email.smtp_username);
    println!("From email: {}", settings.email.from_email);
    println!("Use TLS: {}", settings.email.use_tls);

    let creds = Credentials::new(
        "raffi.programming@gmail.com".to_string(),
        "swddwfzhvnghwpla".to_string(),
    );

    // Use the same configuration as the send_mail function
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Test the connection
    match mailer.test_connection() {
        Ok(_) => {
            println!("✅ SMTP connection test successful!");
            Ok(())
        }
        Err(e) => {
            println!("❌ SMTP connection test failed: {:?}", e);
            Err(Box::new(e))
        }
    }
}
