use miden_objects::{
    accounts::{Account, AccountId},
    assembly::ModuleAst,
    notes::NoteId,
    transaction::ChainMmr,
    utils::serde,
    BlockHeader,
};
use miden_tx::{DataStore, DataStoreError, TransactionExecutor, TransactionInputs};
use rusqlite::{Connection, Result as RusqResult};

pub struct FaucetDataStore {
    faucet: Account,
    block_header: BlockHeader,
    chain_mmr: ChainMmr,
}

impl DataStore for FaucetDataStore {
    fn get_account_code(&self, account_id: AccountId) -> Result<ModuleAst, DataStoreError> {
        Ok(self.faucet)
    }

    fn get_transaction_inputs(
        &self,
        account_id: AccountId,
        block_ref: u32,
        notes: &[NoteId],
    ) -> Result<TransactionInputs, DataStoreError> {
        Ok(self.transaction_inputs)
    }
}

pub fn setup() -> RusqResult<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE faucet (
            id INTEGER PRIMARY KEY
            code TEXT NOT NULL
        )",
        (),
    )?;

    let faucet = FaucetAccount {
        id: "Hello".to_string(),
        code: "World".to_string(),
    };

    conn.execute(
        "INSERT INTO faucet (id, code) VALUES (?1, ?2)",
        (&faucet.id, &faucet.code),
    )?;

    let tx_executor = TransactionExecutor::new(data_store);

    // let mut stmt = conn.prepare(sql)

    Ok(())
}
