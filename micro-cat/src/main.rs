use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define our categories
#[derive(Debug, Serialize, Deserialize, Clone)]
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

// Categorization logic
fn categorize_object(object: &str) -> (Category, f32) {
    let categories_map: HashMap<&str, (Category, f32)> = HashMap::from([
        // Technology keywords
        ("computer", (Category::Technology, 0.9)),
        ("laptop", (Category::Technology, 0.8)),
        ("smartphone", (Category::Technology, 0.9)),
        ("tablet", (Category::Technology, 0.7)),

        // Entertainment keywords
        ("movie", (Category::Entertainment, 0.9)),
        ("film", (Category::Entertainment, 0.8)),
        ("series", (Category::Entertainment, 0.7)),
        ("concert", (Category::Entertainment, 0.8)),
        ("game", (Category::Entertainment, 0.7)),

        // Education keywords
        ("book", (Category::Education, 0.8)),
        ("course", (Category::Education, 0.7)),
        ("class", (Category::Education, 0.6)),
        ("lecture", (Category::Education, 0.7)),

        // Sports keywords
        ("ball", (Category::Sports, 0.6)),
        ("racket", (Category::Sports, 0.7)),
        ("jersey", (Category::Sports, 0.6)),
        ("equipment", (Category::Sports, 0.5)),

        // Food keywords
        ("recipe", (Category::Food, 0.8)),
        ("kitchen", (Category::Food, 0.6)),
        ("cuisine", (Category::Food, 0.7)),

        // Fashion keywords
        ("dress", (Category::Fashion, 0.8)),
        ("shoes", (Category::Fashion, 0.7)),
        ("accessory", (Category::Fashion, 0.6)),

        // Health keywords
        ("fitness", (Category::Health, 0.8)),
        ("medicine", (Category::Health, 0.7)),
        ("wellness", (Category::Health, 0.6)),

        // Travel keywords
        ("passport", (Category::Travel, 0.7)),
        ("luggage", (Category::Travel, 0.6)),
        ("destination", (Category::Travel, 0.5)),
    ]);

    // Convert input to lowercase for case-insensitive matching
    let lowercase_object = object.to_lowercase();

    // Check for direct keyword match
    for (keyword, (category, confidence)) in categories_map.iter() {
        if lowercase_object.contains(keyword) {
            return (category.clone(), *confidence);
        }
    }

    // Default to Technology if no match found
    (Category::Technology, 0.3)
}

// Categorization endpoint
async fn categorize(req: web::Json<CategorizationRequest>) -> impl Responder {
    let (category, confidence) = categorize_object(&req.object);

    HttpResponse::Ok().json(CategorizationResponse {
        object: req.object.clone(),
        category,
        confidence,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/categorize", web::post().to(categorize))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}