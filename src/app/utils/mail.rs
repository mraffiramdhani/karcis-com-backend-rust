use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::app::auth::models::User;

pub async fn send_mail(
    data: User,
    template: String,
) -> Result<<SmtpTransport as Transport>::Ok, <SmtpTransport as Transport>::Error> {
    let host = dotenv::var("MAIL_SMTP_HOST").unwrap();
    let user = dotenv::var("MAIL_SMTP_USERNAME").unwrap();
    let pass = dotenv::var("MAIL_SMTP_PASSWORD").unwrap();
    let creds = Credentials::new(user, pass);
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    let recipient = format!(
        "{} {} {} <{}>",
        &data.title, &data.first_name, &data.last_name, &data.email
    );

    let email = Message::builder()
        .from("Karcis.com <noreply@karcis.com>".parse().unwrap())
        .to(recipient.parse().unwrap())
        .subject("Happy new year")
        .header(ContentType::TEXT_HTML)
        .body(template)
        .unwrap();

    mailer.send(&email)
}
