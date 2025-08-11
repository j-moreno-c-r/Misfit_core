use bitcoin::Transaction;
use misfit_core::transaction::{generator::GenerateTx, random::transaction::TxParams};

#[test]
fn test_generate_tx_valid_random() {
    let tx: Transaction = GenerateTx::valid_random(TxParams::default());

    assert!(tx.input.iter().count() >= 1);
    assert!(tx.output.iter().count() >= 1);
}
