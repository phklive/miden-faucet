use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use cli::Cli;
use configs::{Endpoint, FaucetTopLevelConfig};
use db::FaucetDataStore;
use handlers::get_tokens;
use miden_node_proto::generated::requests::SyncStateRequest;
use miden_node_proto::generated::rpc::api_client::ApiClient;
use miden_objects::accounts::AccountId;
use miden_objects::transaction::ChainMmr;
use rpc::RpcApi;
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
    rpc: RpcApi,
    data_store: FaucetDataStore,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let (faucet_account, faucet_config, rpc) = match &cli.command {
        cli::Command::Init {
            token_symbol,
            decimals,
            max_supply,
            config_path,
        } => {
            // Create faucet and config
            let faucet_account = utils::create_fungible_faucet(token_symbol, decimals, max_supply);
            let faucet_config: FaucetTopLevelConfig = utils::load_config(config_path)
                .extract()
                .expect("Failed to load faucet config.");

            // Setup the rpc
            let client = ApiClient::connect(faucet_config.faucet.rpc_endpoint.to_string())
                .await
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::ConnectionRefused,
                        "Failed to connect to rpc.",
                    )
                })?;
            let rpc = RpcApi { rpc: client };

            // save to the data store
            (faucet_account, faucet_config, rpc)
        }
        cli::Command::Import {
            faucet_path,
            config_path,
        } => {
            // Create faucet and config
            let faucet_account = utils::import_fungible_faucet(faucet_path);
            let faucet_config: FaucetTopLevelConfig = utils::load_config(config_path)
                .extract()
                .expect("Failed to load faucet config.");

            // Setup the rpc
            let client = ApiClient::connect(faucet_config.faucet.rpc_endpoint.to_string())
                .await
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::ConnectionRefused,
                        "Failed to connect to rpc.",
                    )
                })?;
            let rpc = RpcApi { rpc: client };

            // save to the data store
            (faucet_account, faucet_config, rpc)
        }
    };

    let request = SyncStateRequest {
        block_num: 0,
        account_ids: vec![],
        note_tags: vec![],
        nullifiers: vec![],
    };

    let state = rpc.clone().rpc.sync_state(request).await.map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "Failed to sync state",
        )
    })?;

    println!("sync: {:#?}", state);

    let mmr = ChainMmr::

    let data_store = FaucetDataStore {
        account: faucet_account,
        block_header: state.into_inner().block_header,
        block_chain: state.into_inner().mm
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
        rpc,
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
