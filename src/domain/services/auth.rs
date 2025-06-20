use crate::domain::services::{otp::create_otp_service, user::create_user_service};
use crate::infrastructure::email::send_mail;
use crate::infrastructure::email_template::forgot_password::template;
use crate::shared::utils::generator::generate_otp;
use crate::{
    domain::{models::auth::ForgotPasswordPayload, services::AuthService},
    infrastructure::database::PostgresPool,
};
use async_trait::async_trait;

#[async_trait]
impl AuthService for PostgresPool {
    async fn forgot_password(&self, data: &ForgotPasswordPayload) -> Result<(), Box<dyn std::error::Error>> {
        let user_service = create_user_service(self.clone());
        let user = user_service.find_by("email", &data.email).await?;
        if user.is_none() {
            return Err(Box::new(sqlx::Error::RowNotFound));
        }
        let otp = generate_otp();
        let otp_service = create_otp_service(self.clone());
        otp_service.create(&otp.to_string()).await?;
        send_mail(user.unwrap(), "Forgot Password", template(&otp.to_string())).await?;
        Ok(())
    }
}

pub fn create_auth_service(pool: PostgresPool) -> Box<dyn AuthService> {
    Box::new(pool)
}