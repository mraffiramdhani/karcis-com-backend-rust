use async_trait::async_trait;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};

use crate::{
    domain::{
        models::{auth::{LoginPayload, RegisterPayload}, user::User, token::TokenSigning},
        services::UserService,
    },
    infrastructure::database::PostgresPool,
};

#[async_trait]
impl UserService for PostgresPool {
    async fn find_by(&self, field: &str, value: &str) -> Result<Option<User>, sqlx::Error> {
        let query = format!("SELECT * FROM users WHERE {} = $1", field);
        let result = sqlx::query_as::<_, User>(&query)
            .bind(value)
            .fetch_optional(self.pool())
            .await;
        result
    }

    async fn create(&self, user: &RegisterPayload) -> Result<(User, String), sqlx::Error> {
        //* Begin transaction
        let mut tx = self.begin_transaction().await?;
        
        //* Execute operations within transaction
        let result = async{
            //* Create user
            let password_hash = hash(&user.password, 10).unwrap();
            let created_user = sqlx::query_as::<_, User>(
                "INSERT INTO users (first_name, last_name, phone, username, email, password_hash, title, image) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            )
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.phone)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&password_hash)
            .bind(&user.title)
            .bind(&user.image)
            .fetch_one(&mut *tx)
            .await?;
        
            //* Create balance
            let _ = sqlx::query("INSERT INTO balances (user_id, balance) VALUES ($1, 0)",)
                .bind(&created_user.id)
                .execute(&mut *tx)
                .await;

            //* Create token
            let token_data = TokenSigning {
                id: created_user.id.clone(),
                first_name: created_user.first_name.clone(),
                last_name: created_user.last_name.clone(),
                email: created_user.email.clone(),
                role_id: created_user.role_id.clone(),
                exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
            };
            let token = TokenSigning::sign(token_data).unwrap();
            sqlx::query("INSERT INTO revoked_token (token) VALUES ($1)")
                .bind(&token)
                .execute(&mut *tx)
                .await?;

            Ok((created_user, token))
        }.await;

        //* Handle result and commit/rollback transaction
        match result {
            Ok(user) => {
                tx.commit().await?;
                Ok(user)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    async fn login(&self, data: &LoginPayload) -> Result<(User, String), sqlx::Error> {
        let user = self.find_by("username", &data.username).await?;
        if let Some(user) = user {
            if !verify(&data.password, &user.password_hash).unwrap() {
                return Err(sqlx::Error::RowNotFound);
            }
            //* Create token
            let token_data = TokenSigning {
                id: user.id.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
                role_id: user.role_id.clone(),
                exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
            };
            let token = TokenSigning::sign(token_data).unwrap();
            sqlx::query("INSERT INTO revoked_token (token) VALUES ($1)")
                .bind(&token)
                .execute(self.pool())
                .await?;
            Ok((user, token))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

pub fn create_user_service(pool: PostgresPool) -> Box<dyn UserService> {
    Box::new(pool)
}
