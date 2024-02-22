use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use cli::Cli;
use configs::{Endpoint, FaucetTopLevelConfig};
use handlers::get_tokens;
use miden_objects::accounts::AccountId;
use std::io;
use std::net::ToSocketAddrs;

mod cli;
mod configs;
mod db;
mod errors;
mod handlers;
mod rpc;
mod utils;

#[derive(Clone)]
pub struct FaucetState {
    id: AccountId,
    asset_amount: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let (faucet_account, faucet_config) = match &cli.command {
        cli::Command::Init {
            token_symbol,
            decimals,
            max_supply,
            config_path,
        } => {
            let faucet_account = utils::create_fungible_faucet(token_symbol, decimals, max_supply);
            let faucet_config: FaucetTopLevelConfig = utils::load_config(config_path)
                .extract()
                .expect("Failed to load faucet config.");
            // save to the data store
            (faucet_account, faucet_config)
        }
        cli::Command::Import {
            faucet_path,
            config_path,
        } => {
            let faucet_account = utils::import_fungible_faucet(faucet_path);
            let faucet_config: FaucetTopLevelConfig = utils::load_config(config_path)
                .extract()
                .expect("Failed to load faucet config.");
            // save to the data store
            (faucet_account, faucet_config)
        }
    };

    println!(
        "✅ Faucet setup successful, account id: {}",
        faucet_account.id()
    );

    let addr = faucet_config
        .faucet
        .endpoint
        .to_socket_addrs()?
        .next()
        .ok_or(io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "Couldn't resolve server address",
        ))?;

    let Endpoint {
        host,
        port,
        protocol,
    } = faucet_config.faucet.endpoint;

    println!("🚀 Starting server on: {}://{}:{}", protocol, host, port);

    // Instantiate faucet state
    let faucet_state = FaucetState {
        id: faucet_account.id(),
        asset_amount: 100,
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);
        App::new()
            .app_data(web::Data::new(faucet_state.clone()))
            .wrap(cors)
            .service(get_tokens)
            .service(Files::new("/", "src/static").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
