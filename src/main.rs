use actix_web::{ HttpServer, App, HttpResponse, web, http::{ StatusCode, header} };
use serde::{Serialize, Deserialize};

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
    let messages = "Hello";
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
struct Message {
    text: String
}

async fn create(message: web::Form<Message>) -> HttpResponse {
    HttpResponse::build(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .finish()
}