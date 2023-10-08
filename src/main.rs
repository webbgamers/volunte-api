mod model;
use model::*;

use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client,
};

const DB_NAME: &str = "volunte";

#[post("/register")]
async fn register(client: web::Data<Client>, form: web::Json<Register>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection("users");
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(RegisterResponse {
            id: result.inserted_id.as_object_id().unwrap().to_hex(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

#[get("/event")]
async fn get_event(client: web::Data<Client>, form: web::Json<GetEvent>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection::<EventFromBSON>("events");
    let result = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&form.id).unwrap()}, None)
        .await;
    match result {
        Ok(Some(event)) => HttpResponse::Ok().json(event),
        Ok(None) => HttpResponse::NotFound().json(ServerError {
            error: format!("No event found with id {}", form.id),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

#[get("/login")]
async fn login(client: web::Data<Client>, form: web::Json<Login>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection::<UserFromBSON>("users");
    let result = collection.find_one(doc! {"email": &form.email, "password": &form.password}, None).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Forbidden().json(ServerError { error: "Invalid email or password".to_string()}),
        Err(err) => HttpResponse::InternalServerError().json(ServerError { error: err.to_string()})
    }
}

#[get("/user")]
async fn get_user(client: web::Data<Client>, form: web::Json<GetUser>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection::<UserFromBSON>("users");
    let result = collection.find_one(doc! {"_id": ObjectId::parse_str(&form.id).unwrap()}, None).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ServerError { error: format!("No user found with id {}", form.id)}),
        Err(err) => HttpResponse::InternalServerError().json(ServerError { error: err.to_string()})
    }

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
            .service(get_event)
            .service(login)
            .service(get_user)
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
