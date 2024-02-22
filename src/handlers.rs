use crate::{errors::FaucetError, FaucetState};
use actix_web::{get, http::header, web, HttpResponse, Result};
use miden_objects::accounts::AccountId;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    account_id: String,
}

#[get("/get_tokens")]
pub async fn get_tokens(
    req: web::Query<User>,
    state: web::Data<FaucetState>,
) -> Result<HttpResponse> {
    println!("Received a request with account_id: {}", req.account_id);

    let _faucet_account_id = state.id;
    let _fungible_asset_amount = state.asset_amount;

    let _account_id = AccountId::from_hex(req.account_id.as_str())
        .map_err(|err| FaucetError::BadRequest(err.to_string()))?;

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
