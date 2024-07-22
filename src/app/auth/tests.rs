use crate::app::auth::{models, views};
use actix_web::{test, web, App};
use sqlx::PgPool;

#[actix_web::test]
async fn test_login() {
    let connection_string = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create database pool");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/login").route(web::post().to(views::login))),
    )
    .await;

    // Create a test user
    let test_user = models::RegisterUser {
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        phone: "1234567890".to_string(),
    };
    models::User::create(&pool, &test_user)
        .await
        .expect("Failed to create test user");

    // Test valid login
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&models::LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test invalid login
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&models::LoginCredentials {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_register() {
    let connection_string = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create database pool");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/register").route(web::post().to(views::register))),
    )
    .await;

    let new_user = models::RegisterUser {
        first_name: "New".to_string(),
        last_name: "User".to_string(),
        username: "newuser".to_string(),
        email: "new@example.com".to_string(),
        password: "newpassword123".to_string(),
        phone: "9876543210".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&new_user)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify that the user was created in the database
    let user = models::User::find_by_username(&pool, &new_user.username)
        .await
        .expect("Failed to query user");
    assert!(user.is_some());
}

#[actix_web::test]
async fn test_get_profile() {
    let connection_string = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create database pool");
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/profile/{user_id}").route(web::get().to(views::get_profile))),
    )
    .await;

    // Create a test user
    let test_user = models::RegisterUser {
        first_name: "Profile".to_string(),
        last_name: "Test".to_string(),
        username: "profiletest".to_string(),
        email: "profile@example.com".to_string(),
        password: "profilepass123".to_string(),
        phone: "5555555555".to_string(),
    };
    let created_user = models::User::create(&pool, &test_user)
        .await
        .expect("Failed to create test user");

    let req = test::TestRequest::get()
        .uri(&format!("/profile/{}", created_user.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test non-existent user
    let req = test::TestRequest::get().uri("/profile/99999").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

// Note: Logout and refresh_token tests are not implemented
// as their functionalities are not fully implemented in the views.
