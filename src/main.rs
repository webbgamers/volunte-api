mod model;
use model::*;

use std::env;

use futures::stream::TryStreamExt;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongodb::{
    bson::{bson, doc, oid::ObjectId},
    options::FindOptions,
    Client,
};

const DB_MAIN: &str = "volunte";
const COL_EVENTS: &str = "events";
const COL_USERS: &str = "users";

#[post("/register")]
async fn register(client: web::Data<Client>, form: web::Json<Register>) -> HttpResponse {
    let collection = client.database(DB_MAIN).collection(COL_USERS);
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
async fn get_event(client: web::Data<Client>, form: web::Json<GetById>) -> HttpResponse {
    let collection = client
        .database(DB_MAIN)
        .collection::<EventFromBSON>(COL_EVENTS);
    let result = collection
        .find_one(doc! {"_id": bson!(&form.id)}, None)
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
    let collection = client
        .database(DB_MAIN)
        .collection::<UserFromBSON>(COL_USERS);
    let result = collection
        .find_one(
            doc! {"email": &form.email, "password": &form.password},
            None,
        )
        .await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Forbidden().json(ServerError {
            error: "Invalid email or password".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

#[get("/user")]
async fn get_user(client: web::Data<Client>, form: web::Json<GetUser>) -> HttpResponse {
    let collection = client
        .database(DB_MAIN)
        .collection::<UserFromBSON>(COL_USERS);
    let result = collection
        .find_one(doc! {"_id": ObjectId::parse_str(&form.id).unwrap()}, None)
        .await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ServerError {
            error: format!("No user found with id {}", form.id),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

//Bela is very cool for this
#[get("/events")]
async fn get_events_preview(client: web::Data<Client>) -> HttpResponse {
    let collection = client
        .database(DB_MAIN)
        .collection::<EventPreviewFromBSON>(COL_EVENTS);
    let options = FindOptions::builder()
        .projection(doc! {"name": 1, "description": 1, "address": 1})
        .build();
    let result = collection.find(None, options).await;
    match result {
        Ok(cursor) => HttpResponse::Ok().json(cursor.try_collect::<Vec<_>>().await.unwrap()),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

#[get("/user/events")]
async fn user_get_events(client: web::Data<Client>, form: web::Json<GetUser>) -> HttpResponse {
    let collection = client
        .database(DB_MAIN)
        .collection::<TimeSlotPreviewFromBSON>(COL_EVENTS);
    let pipeline = vec![
        doc! { "$unwind": "$timeslots"},
        doc! { "$match": {"timeslots.volunteers": {"$in": ObjectId::parse_str(&form.id).unwrap()}}},
        doc! { "$unset": ["volunteers", "requests"]},
    ];
    let result = collection.aggregate(pipeline, None).await;
    match result {
        Ok(cursor) => HttpResponse::Ok().json(cursor.try_collect::<Vec<_>>().await.unwrap()),
        Err(err) => HttpResponse::InternalServerError().json(ServerError {
            error: err.to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mdb_client = Client::with_uri_str(env::var("URI").expect("Missing Mongo URI."))
        .await
        .expect("Failed to connect to MongoDB.");

    mdb_client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("MongoDB connection test failed.");
    println!("Connected to MongoDB.");

    HttpServer::new(move || {

        let cors = Cors::permissive().max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(mdb_client.clone()))
            .service(register)
            .service(get_event)
            .service(login)
            .service(get_user)
            .service(get_events_preview)
            .service(user_get_events)
    })
    .bind((
        "0.0.0.0",
        env::var("PORT")
            .map(|a| a.parse().expect("Invalid port."))
            .unwrap_or(8080),
    ))?
    .run()
    .await
}
