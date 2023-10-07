mod model;
use model::*;

use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{Client, bson::doc};

const DB_NAME: &str = "volunte";

#[post("/register")]
async fn register(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection("users");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(UserId {
            id: result.inserted_id.as_object_id().unwrap().to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(Error {
            error: err.to_string(),
        }),
    }
}


#[post("/get_data")]
async fn get_data() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mdb_client = Client::with_uri_str(args.get(1).expect("Missing Mongo URI."))
        .await
        .expect("Failed to connect to MongoDB.");

    mdb_client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("MongoDB connection test failed.");
    println!("Connected to MongoDB.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mdb_client.clone()))
            .service(register)
            .service(get_data)
    })
    .bind((
        "127.0.0.1",
        args.get(2)
            .map(|a| a.parse().expect("Invalid port."))
            .unwrap_or(8080),
    ))?
    .run()
    .await
}
