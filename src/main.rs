use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use clap::Parser;
use cli::Cli;
use handlers::get_tokens;

mod cli;
mod errors;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        cli::Command::Start {
            token_symbol,
            decimals,
            max_supply,
        } => println!(
            "token_symbol: {}, decimals: {}, max_supply: {}",
            token_symbol, decimals, max_supply
        ),
    }

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
