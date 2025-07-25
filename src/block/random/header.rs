use bitcoin::{
    block::{Header, Version},
    hashes::Hash,
    Block, BlockHash, CompactTarget, Transaction, TxMerkleNode,
};
use secp256k1::rand::{self, Rng};

use super::{
    bits::RandomBits,
    block::{BlockParams, RandomBlock},
    merkle_root::{MerkleRoot, MerkleRootParams},
    version::RandomVersion,
};

#[derive(Default)]
pub struct HeaderParams {
    pub version: Option<Version>,
    pub prev_blockhash: Option<BlockHash>,
    pub merkle_root: Option<TxMerkleNode>,
    pub time: Option<u32>,
    pub bits: Option<CompactTarget>,
    pub nonce: Option<u32>,
    pub txs: Option<Vec<Transaction>>,
}


pub trait RandomHeader {
    fn random(params: HeaderParams) -> Header;
}

impl RandomHeader for Header {
    fn random(params: HeaderParams) -> Header {
        Header {
            version: params.version.unwrap_or_else(Version::random),
            prev_blockhash: params.prev_blockhash.unwrap_or_else(|| {
                let h_params = HeaderParams {
                    prev_blockhash: Some(BlockHash::all_zeros()),
                    ..Default::default()
                };

                let block_params = BlockParams {
                    header: Some(Header::random(h_params)),
                    ..Default::default()
                };

                let (block, _) = Block::random(block_params); 
                block.block_hash() 
            }),
            merkle_root: params
                .merkle_root
                .unwrap_or_else(|| TxMerkleNode::random(MerkleRootParams { txs: params.txs })),
            time: params
                .time
                .unwrap_or_else(|| rand::thread_rng().gen::<u32>()),
            bits: params.bits.unwrap_or_else(CompactTarget::random),
            nonce: params
                .nonce
                .unwrap_or_else(|| rand::thread_rng().gen::<u32>()),
        }
    }
}
