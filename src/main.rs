use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    web, App, HttpResponse, HttpServer, Result,
};
use derive_more::Display;
use miden_objects::accounts::AccountId;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    account_id: String,
}

#[derive(Debug, Display)]
enum FaucetError {
    BadRequest(String),
    InternalServerError(String),
}

impl error::ResponseError for FaucetError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            FaucetError::BadRequest(msg) => msg,
            FaucetError::InternalServerError(msg) => msg,
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(message.to_owned())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            FaucetError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            FaucetError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/get_tokens")]
async fn get_tokens(req: web::Query<User>) -> Result<HttpResponse> {
    let account_id = AccountId::from_hex(req.account_id.as_str())
        .map_err(|err| FaucetError::BadRequest(err.to_string()))?;

    println!("AccountId: {}", account_id);

    let bytes = "hello".as_bytes();

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header(header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![actix_web::http::header::DispositionParam::Filename(
                "note.mno".to_string(),
            )],
        })
        .body(bytes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(cors)
            .service(get_tokens)
            .service(Files::new("/", "src/static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
