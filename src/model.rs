use mongodb::bson::serde_helpers::{
    deserialize_hex_string_from_object_id, deserialize_rfc3339_string_from_bson_datetime,
    serialize_hex_string_as_object_id, serialize_rfc3339_string_as_bson_datetime,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Register {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ServerError {
    pub error: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetEvent {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetUser {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetEventsPreview {
    //idk yet
}

#[derive(Deserialize, Serialize)]
pub struct UserFromBSON {
    #[serde(
        rename(deserialize = "_id"),
        deserialize_with = "deserialize_hex_string_from_object_id"
    )]
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserFromJSON {
    #[serde(
        rename(serialize = "_id"),
        serialize_with = "serialize_hex_string_as_object_id"
    )]
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct EventFromBSON {
    #[serde(
        rename(deserialize = "_id"),
        deserialize_with = "deserialize_hex_string_from_object_id"
    )]
    pub id: String,
    pub name: String,
    pub address: String,
    pub timeslots: Vec<TimeSlotFromBSON>,
    pub owner: OwnerFromBSON,
}

#[derive(Deserialize, Serialize)]
pub struct EventPreviewFromBSON {
    #[serde(
        rename(deserialize = "_id"),
        deserialize_with = "deserialize_hex_string_from_object_id"
    )]
    pub id: String,
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct EventFromJSON {
    #[serde(
        rename(serialize = "_id"),
        serialize_with = "serialize_hex_string_as_object_id"
    )]
    pub id: String,
    pub name: String,
    pub address: String,
    pub timeslots: Vec<TimeSlotFromJSON>,
    pub owner: OwnerFromJSON,
}

#[derive(Deserialize, Serialize)]
pub struct TimeSlotFromBSON {
    #[serde(deserialize_with = "deserialize_rfc3339_string_from_bson_datetime")]
    pub start: String,
    #[serde(deserialize_with = "deserialize_rfc3339_string_from_bson_datetime")]
    pub end: String,
    pub volunteers: Vec<VolunteerFromBSON>,
    pub requests: Vec<RequestFromBSON>,
}

#[derive(Deserialize, Serialize)]
pub struct TimeSlotFromJSON {
    #[serde(serialize_with = "serialize_rfc3339_string_as_bson_datetime")]
    pub start: String,
    #[serde(serialize_with = "serialize_rfc3339_string_as_bson_datetime")]
    pub end: String,
    pub volunteers: Vec<VolunteerFromJSON>,
    pub requests: Vec<RequestFromJSON>,
}

#[derive(Deserialize, Serialize)]
pub struct VolunteerFromBSON {
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub struct VolunteerFromJSON {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize, Serialize)]
pub struct RequestFromBSON {
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RequestFromJSON {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct OwnerFromBSON {
    #[serde(deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct OwnerFromJSON {
    #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TimeSlotPreviewFromBSON {
    #[serde(deserialize_with = "deserialize_rfc3339_string_from_bson_datetime")]
    pub start: String,
    #[serde(deserialize_with = "deserialize_rfc3339_string_from_bson_datetime")]
    pub end: String,
}
