use crate::{errors::FaucetError, FaucetState};
use actix_web::{get, http::header, web, HttpResponse, Result};
use miden_lib::transaction::memory::FAUCET_STORAGE_DATA_SLOT;
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

    let faucet_account_id = state.id;
    let fungible_asset_amount = state.asset_amount;
    let faucet_initial_balance = state.initial_balance;
    let _account_id = AccountId::from_hex(req.account_id.as_str())
        .map_err(|err| FaucetError::BadRequest(err.to_string()))?;

    let code = format!(
        "
        use.miden::kernels::tx::account
        use.miden::kernels::tx::asset_vault
        use.miden::kernels::tx::memory
        use.miden::kernels::tx::prologue
        use.miden::faucet

        begin
            # mint asset
            exec.prologue::prepare_transaction
            push.{fungible_asset_amount}.0.0.{faucet_account_id}
            exec.faucet::mint

            # assert the correct asset is returned
            push.{fungible_asset_amount}.0.0.{faucet_account_id}
            assert_eqw

            # assert the input vault has been updated
            exec.memory::get_input_vault_root_ptr
            push.{faucet_account_id}
            exec.asset_vault::get_balance
            push.{fungible_asset_amount} assert_eq

            # assert the faucet storage has been updated
            push.{FAUCET_STORAGE_DATA_SLOT}
            exec.account::get_item
            push.{expected_final_storage_amount}
            assert_eq
        end
        ",
        expected_final_storage_amount = faucet_initial_balance + fungible_asset_amount
    );

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
