mod user_repository;

pub use user_repository::*;
pub use amenity_repository::*;
pub use balance_repository::*;

use async_trait::async_trait;
use sqlx::Transaction;
use sqlx::Postgres;

#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn create(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn update(&self, entity: &T) -> Result<T, sqlx::Error>;
    async fn delete(&self, id: ID) -> Result<bool, sqlx::Error>;
}

#[async_trait]
pub trait TransactionalRepository<T, ID> {
    async fn find_by_id_tx(&self, tx: &mut Transaction<'_, Postgres>, id: ID) -> Result<Option<T>, sqlx::Error>;
    async fn create_tx(&self, tx: &mut Transaction<'_, Postgres>, entity: &T) -> Result<T, sqlx::Error>;
    async fn update_tx(&self, tx: &mut Transaction<'_, Postgres>, entity: &T) -> Result<T, sqlx::Error>;
    async fn delete_tx(&self, tx: &mut Transaction<'_, Postgres>, id: ID) -> Result<bool, sqlx::Error>;
} 