use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize)]
struct Recipe {
    name: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
    image_url: String,
}

async fn index() -> impl Responder {
    let recipes = vec![
        Recipe {
            name: String::from("Spaghetti Carbonara"),
            ingredients: vec![
                String::from("Spaghetti"),
                String::from("Eggs"),
                String::from("Pancetta"),
                String::from("Parmesan Cheese"),
            ],
            steps: vec![
                String::from("Boil spaghetti"),
                String::from("Cook pancetta"),
                String::from("Mix eggs with cheese"),
                String::from("Combine everything"),
            ],
            image_url: String::from("https://i.pinimg.com/564x/ad/18/c6/ad18c6a266e334b69e157ee2be9a75cf.jpg"),
        },
        Recipe {
            name: String::from("Chicken Curry"),
            ingredients: vec![
                String::from("Chicken"),
                String::from("Onions"),
                String::from("Garlic"),
                String::from("Curry Powder"),
                String::from("Coconut Milk"),
            ],
            steps: vec![
                String::from("Cook onions and garlic"),
                String::from("Add chicken and cook until browned"),
                String::from("Add curry powder and coconut milk"),
                String::from("Simmer until chicken is cooked through"),
            ],
            image_url: String::from("https://i.pinimg.com/564x/7c/81/60/7c816064a99b416f7f7728c6c83223fc.jpg"),
        },
        Recipe {
            name: String::from("Grilled Cheese Sandwich"),
            ingredients: vec![
                String::from("Bread"),
                String::from("Cheddar Cheese"),
                String::from("Butter"),
            ],
            steps: vec![
                String::from("Butter the bread"),
                String::from("Place cheese between slices of bread"),
                String::from("Grill until golden brown"),
            ],
            image_url: String::from("https://i.pinimg.com/564x/1a/8f/0b/1a8f0b6d5547fb28bc295fa342abcb0c.jpg"),
        },
        Recipe {
            name: String::from("Beef Tacos"),
            ingredients: vec![
                String::from("Ground Beef"),
                String::from("Taco Seasoning"),
                String::from("Taco Shells"),
                String::from("Lettuce"),
                String::from("Cheese"),
                String::from("Tomato"),
            ],
            steps: vec![
                String::from("Cook ground beef with taco seasoning"),
                String::from("Fill taco shells with beef"),
                String::from("Top with lettuce, cheese, and tomato"),
            ],
            image_url: String::from("https://i.pinimg.com/564x/da/1e/e3/da1ee34b9039601edc0a7a8bcccc4ff6.jpg"),
        },
        Recipe {
            name: String::from("Pancakes"),
            ingredients: vec![
                String::from("Flour"),
                String::from("Eggs"),
                String::from("Milk"),
                String::from("Baking Powder"),
                String::from("Sugar"),
            ],
            steps: vec![
                String::from("Mix flour, eggs, and milk"),
                String::from("Add baking powder and sugar"),
                String::from("Cook on a griddle until golden brown"),
            ],
            image_url: String::from("https://i.pinimg.com/564x/d0/bd/dc/d0bddce4efb65033c4422ac3e92c1edf.jpg"),
        },
    ];

    let template = IndexTemplate {
        title: "Cooking Wiki",
        recipes,
    };

    let rendered = template.render().unwrap_or_else(|_| "Error rendering template".to_string());
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
