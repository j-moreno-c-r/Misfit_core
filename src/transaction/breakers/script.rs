use bitcoin::ScriptBuf;

pub fn corrupt_script(script: &ScriptBuf) -> ScriptBuf {
    let mut bytes = script.as_bytes().to_vec();
    if !bytes.is_empty() {
        bytes[0] = bytes[0].wrapping_add(1);
    } else {
        bytes.push(0x51); // Add OP_1 to empty script
    }
    ScriptBuf::from_bytes(bytes)
}