use bitcoin::consensus::encode;
use bitcoin::Transaction;
use misfit_core::block::generator::GenerateBlock;
use misfit_core::block::random::block::BlockParams;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::transaction::random::transaction::TxParams;
pub struct Generator {}

impl Generator {
    // TODO: Implement args
    pub fn block(tx_count: u32) -> String {
        let mut txs: Vec<Transaction> = vec![];
        let mut raw_tx: Vec<String> = vec![];

        for _c in 0..tx_count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();

            txs.push(tx);
            raw_tx.push(raw_transaction);
        }

        let block = GenerateBlock::valid_random(BlockParams {
            header: None,
            txs: Some(txs),
        });

        [
            format!("{:#?} ", block.header),
            format!("Raw txs: {:#?}", raw_tx)
        ]
        .join("\n---\n")
    }

    // TODO: Implement params into transaction generator
    pub fn transaction(count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        [
            format!("Raw Transactions: {:#?}", raw_tx),
            format!("TXIDs: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    pub fn _proces_flags_to_broke(flags: Vec<String>) -> String {
        let mut flags_concateneted = "".to_string();

        for c in flags {
            flags_concateneted += &c;
        }

        format!("When cant process you flags for now {}", flags_concateneted).to_string()
    }
}
