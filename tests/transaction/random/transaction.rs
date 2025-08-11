use bitcoin::{absolute::LockTime, transaction::Version, Transaction};
use misfit_core::transaction::random::{
    locktime::RandomLockTime,
    transaction::{RandomTransacion, TxParams},
    version::RandomVersion,
};

#[test]
fn test_random_transacion() {
    let transaction: Transaction = Transaction::random(TxParams::default());

    assert!(transaction.input.iter().count() >= 1);
    assert!(transaction.output.iter().count() >= 1);
}

#[test]
fn test_random_transacion_with_specified_version() {
    let expected_version: Version = Version::random();

    let transaction: Transaction = Transaction::random(TxParams {
        version: Some(expected_version),
        ..Default::default()
    });

    assert!(transaction.version.eq(&expected_version));
}

#[test]
fn test_random_transacion_with_specified_lock_time() {
    let expected_lock_time: LockTime = LockTime::random();

    let transaction: Transaction = Transaction::random(TxParams {
        lock_time: Some(expected_lock_time),
        ..Default::default()
    });

    assert!(transaction.lock_time.eq(&expected_lock_time));
}

// Needs to implement input and output vector into InputParams

// #[test]
// fn test_random_transacion_with_specified_input() {
//     let expected_inputs: Vec<TxIn> = vec![TxIn::random(InputParams::default())];

//     let transaction: Transaction = Transaction::random(TxParams {
//         input: Some(expected_inputs),
//         ..Default::default()
//     });

//     assert!(transaction.input.eq(&expected_inputs));
// }

// #[test]
// fn test_random_transacion_with_specified_output() {
//     let (output_tx, _) = TxOut::random(OutputParams::default());

//     let expected_outputs: Vec<TxOut> = vec![output_tx];

//     let transaction: Transaction = Transaction::random(TxParams {
//         output: Some(expected_outputs),
//         ..Default::default()
//     });

//     assert!(transaction.output.eq(&expected_outputs));
// }
