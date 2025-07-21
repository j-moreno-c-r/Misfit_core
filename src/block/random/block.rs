use bitcoin::{block::{Header}, Block, Transaction};
use secp256k1::rand::{self, Rng};

use crate::transaction::{
    generator::GenerateTx, 
    random::transaction::{TxParams}
};
use bitcoin::block::Block as BitcoinBlock;
use bitcoin::ScriptBuf;

use super::header::{HeaderParams, RandomHeader};


#[derive(Default)]
pub struct BlockParams {
    pub header: Option<Header>,
    pub txs: Option<Vec<Transaction>>,
    pub height: Option<u32>,
}



pub trait RandomBlock {
    fn random(params: BlockParams) -> (Block, u32);
}


impl RandomBlock for Block {
    fn random(mut params: BlockParams) -> (Block, u32) {
        let  block_height = params.height.unwrap_or_else(|| rand::thread_rng().gen_range(1..10_000_000));
        params.height = Some(block_height);
            let tx_data = params.txs.unwrap_or_else(|| {
            let random = rand::thread_rng().gen_range(1..10);
            let mut txs = vec![];
            for _ in 0..random {
                let mut tx_params = TxParams::default();
                tx_params.block_height = Some(block_height); // NOVO
                let tx_info = GenerateTx::valid_random(tx_params);
                txs.push(tx_info);
            }
            txs
        });

        let has_segwit = tx_data.iter().skip(1).any(|tx| {
            tx.input.iter().any(|i| !i.witness.is_empty())
        });

        let mut tx_data = tx_data;

        if has_segwit && !tx_data.is_empty() {
            {
                let coinbase = &mut tx_data[0];
                if coinbase.input.is_empty() {
                }
                if coinbase.input[0].witness.is_empty() {
                    coinbase.input[0].witness.push(vec![0u8; 32]);
                } else if coinbase.input[0].witness[0].len() != 32 {
                    coinbase.input[0].witness.clear();
                    coinbase.input[0].witness.push(vec![0u8; 32]);
                }
            } 

            let block_tmp = BitcoinBlock {
                header: params.header.clone().unwrap_or_else(|| Header::random(HeaderParams::default())),
                txdata: tx_data.clone(),
            };
            let witness_root = block_tmp.witness_root().unwrap();

            let witness_reserved_value = &tx_data[0].input[0].witness[0];

            let commitment = BitcoinBlock::compute_witness_commitment(
                &witness_root,
                witness_reserved_value,
            );

            let mut script_bytes = vec![
                0x6a, // OP_RETURN
                0x24, // Push 36 bytes
                0xaa, 0x21, 0xa9, 0xed, // magic
            ];
            script_bytes.extend_from_slice(&commitment[..]);

            let script = ScriptBuf::from_bytes(script_bytes);

            tx_data[0].output.push(bitcoin::TxOut {
                value: bitcoin::Amount::from_sat(0),
                script_pubkey: script,
            });
        }

        let header = params.header.unwrap_or_else(|| {
            let mut header_params = HeaderParams::default();
            header_params.txs = Some(tx_data.clone());
            Header::random(header_params)
        });

        let block =  Block {
            header,
            txdata: tx_data,
        };

        (block, block_height)
    }
}