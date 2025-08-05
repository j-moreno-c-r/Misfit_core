use bitcoin::{
    block::{Header, Version},
    hashes::Hash,
    Block, BlockHash, CompactTarget, TxMerkleNode,
};
use misfit_core::block::random::{
    bits::RandomBits,
    block::{BlockParams, RandomBlock},
    header::{HeaderParams, RandomHeader},
    merkle_root::{MerkleRoot, MerkleRootParams},
    version::RandomVersion,
};
use secp256k1::rand::{self, Rng};

#[test]
fn test_random_header() {
    let header: Header = Header::random(HeaderParams::default());

    assert!(header.version.to_consensus() >= 0);
    assert!(header.time >= 1);
    assert!(header.bits.to_consensus() >= 1);
    assert!(header.nonce >= 1);
}

#[test]
fn test_random_header_with_specified_version() {
    let expected_version: Version = Version::random();

    let header: Header = Header::random(HeaderParams {
        version: Some(expected_version),
        ..Default::default()
    });

    assert!(header.version.eq(&expected_version));
}

#[test]
fn test_random_header_with_specified_prev_blockhash() {
    let (block, _) = Block::random(BlockParams {
        header: Some(Header::random(HeaderParams {
            prev_blockhash: Some(BlockHash::all_zeros()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let expected_prev_blockhash: BlockHash = block.block_hash();

    let header: Header = Header::random(HeaderParams {
        prev_blockhash: Some(expected_prev_blockhash),
        ..Default::default()
    });

    assert!(header.prev_blockhash.eq(&expected_prev_blockhash));
}

#[test]
fn test_random_header_with_specified_merkle_root() {
    let expected_merkle_root: TxMerkleNode = TxMerkleNode::random(MerkleRootParams::default());

    let header: Header = Header::random(HeaderParams {
        merkle_root: Some(expected_merkle_root),
        ..Default::default()
    });

    assert!(header.merkle_root.eq(&expected_merkle_root));
}

#[test]
fn test_random_header_with_specified_time() {
    let expected_time: u32 = rand::thread_rng().gen::<u32>();

    let header: Header = Header::random(HeaderParams {
        time: Some(expected_time),
        ..Default::default()
    });

    assert!(header.time.eq(&expected_time));
}

#[test]
fn test_random_header_with_specified_bits() {
    let expected_bits: CompactTarget = CompactTarget::random();

    let header: Header = Header::random(HeaderParams {
        bits: Some(expected_bits),
        ..Default::default()
    });

    assert!(header.bits.eq(&expected_bits));
}

#[test]
fn test_random_header_with_specified_nonce() {
    let expected_nonce: u32 = rand::thread_rng().gen::<u32>();

    let header: Header = Header::random(HeaderParams {
        nonce: Some(expected_nonce),
        ..Default::default()
    });

    assert!(header.nonce.eq(&expected_nonce));
}
