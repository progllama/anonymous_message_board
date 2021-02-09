use actix_web::{ HttpServer, App, HttpResponse, web, http::{ StatusCode, header} };
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use schema::messages;
use models::{ Message, NewMessage };

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::new( || {
        App::new()
        .route("/", web::get().to(index))
        .route("/create", web::post().to(create))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    let messages = get_all_message().await;

    let messages = messages.iter().map(|msg| {
        format!("{} {}<br>", msg.text, msg.create_at)
    })
    .collect::<Vec<_>>()
    .concat();

    HttpResponse::Ok()
        .body(format!("
            <!DOCTYPE html>
            <html>
                <head>
            
                </head>
                <body>
                    {}
                    <form action=\"create\" method=\"POST\" name=\"Message\">
                        <textarea name=\"text\"></textarea>
                        <input type=\"submit\">
                    </form>
                </body>
            </html>"
            , messages)
        )
}

#[derive(Serialize, Deserialize)]
struct MessageForm {
    text: String
}

async fn create(message: web::Form<MessageForm>) -> HttpResponse {
    create_message(&message.text).await;
    HttpResponse::build(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .finish()
}

async fn get_all_message() -> Vec<Message> {
    use schema::messages::dsl::*;
    let con = establish_connection();
    messages.select((id, text, create_at)).load::<Message>(&con).unwrap()
}

async fn create_message(text: &String) {
    let connection = establish_connection();

    let new_msg = NewMessage {
        text: &text,
    };

    let _ = diesel::insert_into(messages::table)
        .values(new_msg)
        .execute(&connection);
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}