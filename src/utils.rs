use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use figment::{
    providers::{Format, Toml},
    Figment,
};
use miden_lib::{accounts::faucets::create_basic_fungible_faucet, AuthScheme};
use miden_objects::{
    accounts::{Account, AccountData},
    assets::TokenSymbol,
    crypto::dsa::rpo_falcon512::KeyPair,
    utils::serde::Deserializable,
    Felt,
};

/// Creates a Miden fungible faucet from arguments
pub fn create_fungible_faucet(token_symbol: &String, decimals: &u8, max_supply: &u64) -> Account {
    let token_symbol = TokenSymbol::new(token_symbol).expect("Failed to parse token_symbol.");

    // Instantiate init_seed
    let init_seed: [u8; 32] = [0; 32];

    // Instantiate keypair and authscheme
    let auth_seed: [u8; 40] = [0; 40];
    let keypair = KeyPair::from_seed(&auth_seed).expect("Failed to generate keypair.");
    let auth_scheme = AuthScheme::RpoFalcon512 {
        pub_key: keypair.public_key(),
    };

    let (account, _) = create_basic_fungible_faucet(
        init_seed,
        token_symbol,
        decimals.clone(),
        Felt::try_from(max_supply.clone()).expect("Max_supply is outside of the possible range."),
        auth_scheme,
    )
    .expect("Failed to generate faucet account.");

    account
}

/// Imports a Miden fungible faucet from a file
pub fn import_fungible_faucet(path: &PathBuf) -> Account {
    let path = Path::new(path);
    let mut file = File::open(path).expect("Failed to open file.");

    let mut contents = Vec::new();
    let _ = file.read_to_end(&mut contents);

    let account_data =
        AccountData::read_from_bytes(&contents).expect("Failed to deserialize faucet from file.");
    account_data.account
}

/// Loads the user configuration.
///
/// This function will look for the configuration file at the provided path. If the path is
/// relative, searches in parent directories all the way to the root as well.
///
/// The above configuration options are indented to support easy of packaging and deployment.
pub fn load_config(config_file: &Path) -> Figment {
    Figment::from(Toml::file(config_file))
}
