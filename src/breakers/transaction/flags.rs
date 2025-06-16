// Define available invalidation flags
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

impl InvalidationFlag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "version" => Some(Self::Version),
            "input-txid" | "txid" => Some(Self::InputTxid),
            "input-vout" | "vout" => Some(Self::InputVout),
            "input-script" | "script-sig" => Some(Self::InputScriptSig),
            "input-sequence" | "sequence" => Some(Self::InputSequence),
            "output-amount" | "amount" => Some(Self::OutputAmount),
            "output-script" | "script-pubkey" => Some(Self::OutputScriptPubKey),
            "witness" | "witness-data" => Some(Self::WitnessData),
            "locktime" => Some(Self::Locktime),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}
