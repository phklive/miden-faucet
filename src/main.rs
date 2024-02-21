use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use clap::Parser;
use cli::Cli;
use config::{Endpoint, FaucetTopLevelConfig};
use handlers::get_tokens;
use std::io;
use std::net::ToSocketAddrs;

mod cli;
mod config;
mod errors;
mod handlers;
mod utils;

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

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);
        App::new()
            .wrap(cors)
            .service(get_tokens)
            .service(Files::new("/", "src/static").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}
