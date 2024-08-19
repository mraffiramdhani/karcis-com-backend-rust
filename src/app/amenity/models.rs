use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Amenity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
}

impl Amenity {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM amenities")
            .fetch_all(pool)
            .await;
        result
    }

    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM amenities WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await;
        result
    }

    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        amenity: &CreateAmenity,
    ) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>(
            "INSERT INTO amenities (name, description, icon) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&amenity.name)
        .bind(&amenity.description)
        .bind(&amenity.icon)
        .fetch_one(&mut **transaction)
        .await;
        result
    }

    pub async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        amenity: &UpdateAmenity,
    ) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Self>(
            "UPDATE amenities SET name = $1, description = $2, icon = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&amenity.name)
        .bind(&amenity.description)
        .bind(&amenity.icon)
        .bind(&amenity.id)
        .fetch_one(&mut **transaction)
        .await;
        result
    }

    pub async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        id: i64,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("DELETE FROM amenities WHERE id = $1")
            .bind(id)
            .execute(&mut **transaction)
            .await;
        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAmenity {
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAmenity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
}
