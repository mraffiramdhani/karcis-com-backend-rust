use crate::{
    app::{
        amenity::models::{Amenity, CreateAmenity, UpdateAmenity},
        utils::standard_response::StandardResponse,
    },
    db::DbPool,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_amenities(pool: web::Data<DbPool>) -> impl Responder {
    match Amenity::get_all(&pool).await {
        Ok(amenities) => HttpResponse::Ok().json(StandardResponse::ok(
            amenities,
            Some("Amenities found.".into()),
        )),
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Get Amenities Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn get_amenity_by_id(pool: web::Data<DbPool>, id: web::Path<i64>) -> impl Responder {
    match Amenity::find_by_id(&pool, id.into_inner()).await {
        Ok(Some(amenity)) => {
            HttpResponse::Ok().json(StandardResponse::ok(amenity, Some("Amenity found.".into())))
        }
        Ok(None) => HttpResponse::NotFound()
            .json(StandardResponse::<()>::error("Amenity not found.".into())),
        Err(e) => HttpResponse::InternalServerError().json(StandardResponse::<()>::error(format!(
            "Get Amenity Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn create_amenity(
    pool: web::Data<DbPool>,
    amenity: web::Json<CreateAmenity>,
) -> impl Responder {
    let mut transaction = pool.begin().await.unwrap();
    match Amenity::create(&mut transaction, &amenity).await {
        Ok(new_amenity) => HttpResponse::Created().json(StandardResponse::ok(
            new_amenity,
            Some("Amenity created.".into()),
        )),
        Err(e) => HttpResponse::BadRequest().json(StandardResponse::<()>::error(format!(
            "Create Amenity Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn update_amenity(
    pool: web::Data<DbPool>,
    amenity: web::Json<UpdateAmenity>,
) -> impl Responder {
    let mut transaction = pool.begin().await.unwrap();
    match Amenity::update(&mut transaction, &amenity).await {
        Ok(updated_amenity) => HttpResponse::Ok().json(StandardResponse::ok(
            updated_amenity,
            Some("Amenity updated.".into()),
        )),
        Err(e) => HttpResponse::BadRequest().json(StandardResponse::<()>::error(format!(
            "Update Amenity Error: {}",
            e.to_string()
        ))),
    }
}

pub async fn delete_amenity(pool: web::Data<DbPool>, id: web::Path<i64>) -> impl Responder {
    let mut transaction = pool.begin().await.unwrap();
    match Amenity::delete(&mut transaction, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::BadRequest().json(StandardResponse::<()>::error(format!(
            "Delete Amenity Error: {}",
            e.to_string()
        ))),
    }
}
