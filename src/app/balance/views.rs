use crate::app::utils::standard_response::StandardResponse;

use super::models::{Balance, BalanceHistory, CreateBalanceHistory, UpdateBalance};
use actix_web::{web, HttpResponse, Responder};
use bigdecimal::BigDecimal;
use sqlx::PgPool;

pub async fn get_balance(db: web::Data<PgPool>, path: web::Path<i64>) -> impl Responder {
    let balance_id = path.into_inner();

    match Balance::get_by_user(&db, balance_id).await {
        Ok(balance) => HttpResponse::Ok().json(StandardResponse::ok(Some(balance), None)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_balance_histories(db: web::Data<PgPool>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();

    match BalanceHistory::get_by_user(&db, user_id).await {
        Ok(balance_histories) => {
            HttpResponse::Ok().json(StandardResponse::ok(Some(balance_histories), None))
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_balance(
    db: web::Data<PgPool>,
    path: web::Path<i64>,
    body: web::Json<UpdateBalance>,
) -> impl Responder {
    let balance_id = path.into_inner();

    match Balance::get(&db, &balance_id).await {
        Ok(balance) => {
            let update_balance = body.into_inner();
            let top_up_nominal = if update_balance.balance > balance.balance {
                &update_balance.balance - &balance.balance
            } else {
                BigDecimal::from(0)
            };
            match Balance::update(&db, &balance_id, &update_balance).await {
                Ok(_) => match BalanceHistory::create(
                    &db,
                    CreateBalanceHistory {
                        user_id: balance.user_id.clone(),
                        balance_id: balance.id.clone(),
                        balance: balance.balance.clone(),
                        top_up: top_up_nominal.clone(),
                    },
                )
                .await
                {
                    Ok(_) => HttpResponse::Ok().json(StandardResponse::ok(Some(&balance), None)),
                    Err(_) => HttpResponse::NotFound().finish(),
                },
                Err(_) => HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
