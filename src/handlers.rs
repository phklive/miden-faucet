use crate::errors::FaucetError;
use actix_web::{get, http::header, web, HttpResponse, Result};
use miden_objects::accounts::AccountId;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    account_id: String,
}

#[get("/get_tokens")]
pub async fn get_tokens(req: web::Query<User>) -> Result<HttpResponse> {
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
