use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlx::mysql::MySqlPool;
use dotenv::dotenv;

// Define our categories
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum Category {
    Technology,
    Entertainment,
    Education,
    Sports,
    Food,
    Fashion,
    Health,
    Travel,
}

// Struct for categorization request
#[derive(Deserialize)]
struct CategorizationRequest {
    object: String,
}

// Struct for categorization response
#[derive(Serialize)]
struct CategorizationResponse {
    object: String,
    category: Category,
    confidence: f32,
}

// Struct to store database connection
struct AppState {
    db: MySqlPool,
}

// Categorization logic
fn categorize_object(object: &str) -> (Category, f32) {
    let categories_map: HashMap<&str, (Category, f32)> = HashMap::from([
        ("computer", (Category::Technology, 0.9)),
        ("laptop", (Category::Technology, 0.8)),
        ("smartphone", (Category::Technology, 0.9)),
        ("tablet", (Category::Technology, 0.7)),
        ("movie", (Category::Entertainment, 0.9)),
        ("film", (Category::Entertainment, 0.8)),
        ("series", (Category::Entertainment, 0.7)),
        ("concert", (Category::Entertainment, 0.8)),
        ("game", (Category::Entertainment, 0.7)),
        ("book", (Category::Education, 0.8)),
        ("course", (Category::Education, 0.7)),
        ("class", (Category::Education, 0.6)),
        ("lecture", (Category::Education, 0.7)),
        ("ball", (Category::Sports, 0.6)),
        ("racket", (Category::Sports, 0.7)),
        ("jersey", (Category::Sports, 0.6)),
        ("equipment", (Category::Sports, 0.5)),
        ("recipe", (Category::Food, 0.8)),
        ("kitchen", (Category::Food, 0.6)),
        ("cuisine", (Category::Food, 0.7)),
        ("dress", (Category::Fashion, 0.8)),
        ("shoes", (Category::Fashion, 0.7)),
        ("accessory", (Category::Fashion, 0.6)),
        ("fitness", (Category::Health, 0.8)),
        ("medicine", (Category::Health, 0.7)),
        ("wellness", (Category::Health, 0.6)),
        ("passport", (Category::Travel, 0.7)),
        ("luggage", (Category::Travel, 0.6)),
        ("destination", (Category::Travel, 0.5)),
    ]);

    let lowercase_object = object.to_lowercase();
    for (keyword, (category, confidence)) in categories_map.iter() {
        if lowercase_object.contains(keyword) {
            return (category.clone(), *confidence);
        }
    }

    (Category::Technology, 0.3)
}

// Categorization endpoint
async fn categorize(
    req: web::Json<CategorizationRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (category, confidence) = categorize_object(&req.object);

    // Save result to database
    match sqlx::query(
        "INSERT INTO categorizations (object, category, confidence) VALUES (?, ?, ?)",
    )
    .bind(&req.object)
    .bind(format!("{:?}", category)) // Convert enum to string
    .bind(confidence)
    .execute(&data.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(CategorizationResponse {
            object: req.object.clone(),
            category,
            confidence,
        }),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categorizations (
            id INT AUTO_INCREMENT PRIMARY KEY,
            object VARCHAR(255) NOT NULL,
            category VARCHAR(50) NOT NULL,
            confidence FLOAT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("✅ Tabla 'categorizations' creada exitosamente");

    println!("🚀 Servidor iniciando en http://127.0.0.1:8081");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .route("/categorize", web::post().to(categorize))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use sqlx::mysql::MySqlPoolOptions;
    use serde_json::json;
    use dotenv::dotenv; // Importamos dotenv para cargar el archivo .env
    use std::env; // Para acceder a las variables de entorno

    #[actix_web::test]
    async fn test_categorize_object() {
        dotenv().ok(); // Carga el archivo .env
        let (category, confidence) = categorize_object("computer");
        assert_eq!(category, Category::Technology);
        assert!((confidence - 0.9).abs() < f32::EPSILON);

        let (category, confidence) = categorize_object("recipe");
        assert_eq!(category, Category::Food);
        assert!((confidence - 0.8).abs() < f32::EPSILON);

        let (category, confidence) = categorize_object("unknown");
        assert_eq!(category, Category::Technology);
        assert!((confidence - 0.3).abs() < f32::EPSILON);
    }

    #[actix_web::test]
    async fn test_categorize_endpoint() {
        dotenv().ok(); // Carga el archivo .env

        // Obtén la URL de conexión desde el archivo .env
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");

        // Conecta al pool de la base de datos usando la URL de conexión
        let pool = MySqlPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to the database");

        let app_state = web::Data::new(AppState { db: pool });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/categorize", web::post().to(categorize)),
        )
        .await;

        // Simula una solicitud
        let payload = json!({
            "object": "computer"
        });

        let req = test::TestRequest::post()
            .uri("/categorize")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["object"], "computer");
        assert_eq!(body["category"], "Technology");
        assert_eq!(body["confidence"], 0.9);
    }
}
