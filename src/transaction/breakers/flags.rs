use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InvalidationFlag {
    Version,    
    InputTxid,
    InputVout,
    InputScriptSig,
    InputSequence,
    OutputAmount,
    OutputScriptPubKey,
    WitnessData,
    Locktime,
    All,
}

impl FromStr for InvalidationFlag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "version" => Ok(Self::Version),
            "input-txid" | "txid" => Ok(Self::InputTxid),
            "input-vout" | "vout" => Ok(Self::InputVout),
            "input-script" | "script-sig" => Ok(Self::InputScriptSig),
            "input-sequence" | "sequence" => Ok(Self::InputSequence),
            "output-amount" | "amount" => Ok(Self::OutputAmount),
            "output-script" | "script-pubkey" => Ok(Self::OutputScriptPubKey),
            "witness" | "witness-data" => Ok(Self::WitnessData),
            "locktime" => Ok(Self::Locktime),
            "all" => Ok(Self::All),
            _ => Err(()),
        }
    }
}