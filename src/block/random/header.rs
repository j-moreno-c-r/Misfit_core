use bitcoin::{
    block::{Header, Version},
    hashes::Hash,
    Block, BlockHash, CompactTarget, TxMerkleNode,
};
use secp256k1::rand::{self, Rng};

use super::{
    bits::RandomBits,
    block::{BlockParams, RandomBlock},
    merkle_root::RandomMerkleRoot,
    version::RandomVersion,
};

pub struct HeaderParams {
    pub version: Option<Version>,
    pub prev_blockhash: Option<BlockHash>,
    pub merkle_root: Option<TxMerkleNode>,
    pub time: Option<u32>,
    pub bits: Option<CompactTarget>,
    pub nonce: Option<u32>,
}

impl Default for HeaderParams {
    fn default() -> Self {
        HeaderParams {
            version: None,
            prev_blockhash: None,
            merkle_root: None,
            time: None,
            bits: None,
            nonce: None,
        }
    }
}

pub trait RandomHeader {
    fn random(params: HeaderParams) -> Header;
}

impl RandomHeader for Header {
    fn random(params: HeaderParams) -> Header {
        Header {
            version: params.version.unwrap_or_else(|| Version::random()),
            prev_blockhash: params.prev_blockhash.unwrap_or_else(|| {
                let mut h_params = HeaderParams::default();
                h_params.prev_blockhash = Some(BlockHash::all_zeros());

                let mut block_params = BlockParams::default();
                block_params.header = Some(Header::random(h_params));

                Block::random(block_params).block_hash()
            }),
            merkle_root: params.merkle_root.unwrap_or_else(|| TxMerkleNode::random()), // Get from txs
            time: params
                .time
                .unwrap_or_else(|| rand::thread_rng().gen::<u32>()),
            bits: params.bits.unwrap_or_else(|| CompactTarget::random()),
            nonce: params
                .nonce
                .unwrap_or_else(|| rand::thread_rng().gen::<u32>()),
        }
    }
}
