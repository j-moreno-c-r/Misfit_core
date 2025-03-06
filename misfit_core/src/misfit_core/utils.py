import hashlib
import sys
import random
from subprocess import run

import time
import struct
def take_time():
    # Get the current Unix timestamp
    current_time = int(time.time())

    # Ensure the timestamp is valid (this is a simplified check)
    # In practice, you'd compare it to the median time of the last 11 blocks.
    if current_time < 0 or current_time > 0xFFFFFFFF:
        raise ValueError("Invalid timestamp")

    # Encode the timestamp as a 4-byte little-endian integer
    nTime = struct.pack('<I', current_time)
    return(current_time)


# Swap Endian function
def swap_endian(data):
    return ''.join([data[i:i+2] for i in range(0, len(data), 2)][::-1])

# Merkle Root Function
def merkleroot_binary(txids):
    # Stop recursion if there is only one hash value left, because that's the merkle root.
    if len(txids) == 1:
        return txids[0]

    # Create the new array of hashes
    pair_hashes = []
    while len(txids) > 0:
        if len(txids) >= 2:
            # Get first two
            pair_first = txids[0]
            pair_second = txids[1]

            # Hash them (double SHA256)
            pair = pair_first + pair_second
            pair_hash = hashlib.sha256(hashlib.sha256(pair).digest()).digest()
            pair_hashes.append(pair_hash)

            # Remove those two from the array
            txids = txids[2:]

        if len(txids) == 1:
            # Get the first one twice
            pair_first = txids[0]
            pair_second = txids[0]

            # Hash it with itself (double SHA256)
            pair = pair_first + pair_second
            pair_hash = hashlib.sha256(hashlib.sha256(pair).digest()).digest()
            pair_hashes.append(pair_hash)

            # Remove it from the array
            txids = txids[1:]

    # Recursion bit. Re-apply this function to the new array of hashes we've just created.
    return merkleroot_binary(pair_hashes)

def merkleroot(txids):
    # Convert txids into big endian (BE), because that's the format they need to be in to get the merkle root.
    txids_be = [swap_endian(txid) for txid in txids]

    # Now convert each of these txids into binary, because the hash function wants the binary value, not the hex.
    txids_be_binary = [bytes.fromhex(txid_be) for txid_be in txids_be]

    # Work out the merkle root (in binary) using that lovely recursive function above.
    merkleroot_bin = merkleroot_binary(txids_be_binary)

    # Convert the merkle root into hexadecimal and little-endian, because that's how it's stored in the block header.
    merkleroot_hex = swap_endian(merkleroot_bin.hex())

    # Return it :)
    return merkleroot_hex

def reverse_bytes(hex_string):
    """
    Reverses the bytes of a hexadecimal string.

    Args:
        hex_string (str): A hexadecimal string (e.g., "abcdef1234567890").

    Returns:
        str: The hexadecimal string with its bytes reversed (e.g., "9078563412efcdab").
    """
    # Split the hex string into pairs of characters (bytes)
    byte_pairs = [hex_string[i:i+2] for i in range(0, len(hex_string), 2)]
    
    # Reverse the list of byte pairs
    reversed_byte_pairs = byte_pairs[::-1]
    
    # Join the reversed byte pairs into a single string
    reversed_hex_string = ''.join(reversed_byte_pairs)
    
    return reversed_hex_string

"""
txids = ['dba724cbd65bb986b2c2111061e0d86af9d5ad0bdd78fbefdee33a1eefec4ec3',
      '21a2ea05ed5b7235c9b6bc82e457f184c9e21ca078f50e93c526258a91449b4c'] 

# Print Result
root = merkleroot(txids)
print(root)
print(reverse_bytes(root))
"""


def randomize(str) -> str:
    return random.randbytes(len(bytes.fromhex(str))).hex()


def bcli(cmd: str):
    res = run(
        ["bitcoin-cli", "-regtest"] + cmd.split(" "), capture_output=True, encoding="utf-8")
    if res.returncode == 0:
        return res.stdout.strip()
    else:
        raise Exception(res.stderr.strip())
    

