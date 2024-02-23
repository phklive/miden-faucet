use miden_objects::{
    accounts::{Account, AccountId},
    assembly::ModuleAst,
    notes::NoteId,
    transaction::{ChainMmr, InputNotes},
    utils::serde::{self, Deserializable, Serializable},
    BlockHeader,
};
use miden_tx::{DataStore, DataStoreError, TransactionExecutor, TransactionInputs};
use rusqlite::{Connection, Result as RusqResult};

// If it is a new account it needs to have a valid account seed
// For now the chain_mmr is not serde
#[derive(Clone)]
pub struct FaucetDataStore {
    pub account: Account,
    pub block_header: BlockHeader,
    pub block_chain: ChainMmr,
}

impl DataStore for FaucetDataStore {
    fn get_account_code(&self, account_id: AccountId) -> Result<ModuleAst, DataStoreError> {
        Ok(self.account.code().module().clone())
    }

    fn get_transaction_inputs(
        &self,
        _account_id: AccountId,
        _block_ref: u32,
        _notes: &[NoteId],
    ) -> Result<TransactionInputs, DataStoreError> {
        let input_notes = InputNotes::new(vec![]).map_err(|err| {
            DataStoreError::InternalError(format!("Failed to create input notes: {}", err))
        })?;
        let tx_inputs = TransactionInputs::new(
            self.account.clone(),
            None,
            self.block_header,
            self.block_chain.clone(),
            input_notes,
        )
        .map_err(|err| {
            DataStoreError::InternalError(format!("Failed to create transaction inputs: {}", err))
        })?;
        Ok(tx_inputs)
    }
}

//
// pub fn setup() -> RusqResult<()> {
//     let conn = Connection::open_in_memory()?;
//
//     conn.execute(
//         "CREATE TABLE faucet (
//             id INTEGER PRIMARY KEY
//             code TEXT NOT NULL
//         )",
//         (),
//     )?;
//
//     let faucet = FaucetAccount {
//         id: "Hello".to_string(),
//         code: "World".to_string(),
//     };
//
//     conn.execute(
//         "INSERT INTO faucet (id, code) VALUES (?1, ?2)",
//         (&faucet.id, &faucet.code),
//     )?;
//
//     let tx_executor = TransactionExecutor::new(data_store);
//
//     // let mut stmt = conn.prepare(sql)
//
//     Ok(())
// }
//
//
// SERIALIZATION
// ================================================================================================
//
// impl Serializable for FaucetDataStore {
//     fn write_into<W: serde::ByteWriter>(&self, target: &mut W) {
//         let FaucetDataStore {
//             faucet,
//             block_header,
//             // chain_mmr,
//         } = self;
//         faucet.write_into(target);
//         block_header.write_into(target);
//         // chain_mmr.write_into(target);
//     }
// }

// impl Deserializable for FaucetDataStore {
//     fn read_from<R: serde::ByteReader>(
//         source: &mut R,
//     ) -> Result<Self, serde::DeserializationError> {
//         let faucet = Account::read_from(source)?;
//         let block_header = BlockHeader::read_from(source)?;
//         Ok(Self {
//             faucet,
//             block_header,
//         })
//     }
// }
