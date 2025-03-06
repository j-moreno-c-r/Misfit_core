from src.utils import bcli, randomize
import json
import sys


class CreateTx:
    def __init__(self, **kwargs):
        # Transaction
        self.tx_version: bool = kwargs.get("tx_version", False)
        self.tx_locktime: bool = kwargs.get("tx_locktime", False)

        # Inputs
        # TODO: self.tx_in_count: int = kwargs.get("tx_in_count", 0)
        # TODO: self.invalid_tx_in_count: int = kwargs.get("invalid_tx_in_count", 0)

        self.tx_in_txid: bool = kwargs.get("tx_in_txid", False)
        # TODO: self.tx_in_vout: bool = kwargs.get("tx_in_vout", False)
        self.tx_in_script: bool = kwargs.get("tx_in_script", False)
        self.tx_in_sequence: bool = kwargs.get("tx_in_sequence", False)

        # Outputs
        # TODO: self.tx_out_count: int = kwargs.get("tx_out_count", 0)
        # TODO: self.invalid_tx_out_count: int = kwargs.get("invalid_tx_out_count", 0)

        # TODO: self.tx_out_amount: bool = kwargs.get("tx_out_amount", False)
        # TODO: self.tx_out_script_size: bool = kwargs.get("tx_out_script_size", False)
        self.tx_out_script: bool = kwargs.get("tx_out_script", False)

        # Witness
        # TODO: self.tx_witness_count: int = kwargs.get("tx_witness_count", 0)
        # TODO: self.invalid_tx_witness_count: int = kwargs.get("invalid_tx_witness_count", 0)

        # TODO: self.tx_witness_size: bool = kwargs.get("tx_witness_size", False)
        self.tx_witness_item: bool = kwargs.get("tx_witness_item", False)

    def create_misfit_transaction(self) -> str:
        raw_tx = self.create_valid_tx()
        decoded_tx = self.split_transaction(raw_tx)
        misfit_tx = self.replace_misfit(decoded_tx)
        return self.assemble_transaction(decoded_tx)

    def create_valid_tx(self) -> str:
        # Check if bitcoind -regtest is running
        try:
            bcli("getblockchaininfo")
        except:
            sys.exit("No nodes detected, please run `bitcoind -regtest`.")

        # Create a wallet
        try:
            bcli("createwallet misfit-wallet")
        except:
            pass

        # Loads misfit-wallet
        try:
            bcli("loadwallet misfit-wallet")
        except:
            pass

        # Generate new address for wallet
        addr = bcli("getnewaddress misfit-wallet")

        # Generate funds to the address
        blockhash = json.loads(bcli(f"generatetoaddress 101 {addr}"))[0]

        # Get block data
        block = json.loads(bcli(f"getblock {blockhash} 2"))

        # Get transaction from block
        tx = block['tx'][0]

        # Create valid raw transaction
        tx_in = json.dumps([{
            "txid": tx['txid'],
            "vout": tx['vout'][0]['n']
        }], separators=(',', ':'))

        amount = tx['vout'][0]['value'] - 0.01

        tx_out = json.dumps([{
            bcli("getnewaddress"): amount
        }], separators=(',', ':'))

        raw_tx = bcli(f'createrawtransaction {tx_in} {tx_out}')

        # Sign transaction
        signed_tx = json.loads(
            bcli(f'signrawtransactionwithwallet {raw_tx}'))['hex']

        # Check if is valid tx
        json.loads(bcli(f'testmempoolaccept ["{signed_tx}"]'))[0]

        return raw_tx

    def split_transaction(self, raw_tx: str) -> object:
        offset = 0
        txns = bytes.fromhex(raw_tx)

        version = txns[offset:offset+4]
        offset += 4

        marker = txns[offset:offset+1]
        offset += 1

        flag = txns[offset:offset+1]
        offset += 1

        tx_in_count = txns[offset:offset+1]
        offset += 1

        tx_in = []
        for _ in range(int.from_bytes(tx_in_count)):
            previous_output = txns[offset:offset+36]
            offset += 36

            script_length = txns[offset:offset+1]
            offset += 1

            signature_script = txns[offset:offset +
                                    int.from_bytes(script_length)]
            offset += int.from_bytes(script_length)

            sequence = txns[offset:offset+4]
            offset += 4

            tx_in.append({
                "previous_output": previous_output.hex(),
                "script_length": script_length.hex(),
                "signature_script": signature_script.hex(),
                "sequence": sequence.hex()
            })

        tx_out_count = txns[offset:offset+1]
        offset += 1

        tx_out = []
        for _ in range(int.from_bytes(tx_out_count)):
            value = txns[offset:offset+8]
            offset += 8

            pk_script_length = txns[offset:offset+1]
            offset += 1

            pk_script = txns[offset:offset+int.from_bytes(pk_script_length)]
            offset += int.from_bytes(pk_script_length)

            tx_out.append({
                "value": value.hex(),
                "pk_script_length": pk_script_length.hex(),
                "pk_script": pk_script.hex()
            })

        witness_count = txns[offset:offset+1]
        offset += 1

        witness = []
        for _ in range(int.from_bytes(witness_count)):
            size = txns[offset:offset+1]
            offset += 1

            item = txns[offset:offset+int.from_bytes(size)]
            offset += int.from_bytes(size)

            witness.append({
                "size": size.hex(),
                "item": item.hex()
            })

        lock_time = txns[offset:offset+4]
        offset += 4

        return {
            "version": version.hex(),
            "marker": marker.hex(),
            "flag": flag.hex(),
            "tx_in_count": tx_in_count.hex(),
            "tx_in": tx_in,
            "tx_out_count": tx_out_count.hex(),
            "tx_out": tx_out,
            "witness": witness,
            "locktime": lock_time.hex()
        }

    def replace_misfit(self, decoded_tx: object) -> object:
        if self.tx_version:
            decoded_tx['version'] = randomize(decoded_tx['version'])

        if self.tx_locktime:
            decoded_tx['locktime'] = randomize(decoded_tx['locktime'])

        if self.tx_in_txid:
            for x in decoded_tx['tx_in']:
                x['previous_output'] = randomize(x['previous_output'])

        if self.tx_in_script:
            for x in decoded_tx['tx_in']:
                x['signature_script'] = randomize(x['signature_script'])

        if self.tx_in_sequence:
            for x in decoded_tx['tx_in']:
                x['sequence'] = randomize(x['sequence'])

        if self.tx_out_script:
            for x in decoded_tx['tx_out']:
                x['pk_script'] = randomize(x['pk_script'])

        if self.tx_witness_item:
            for x in decoded_tx['witness']:
                x['item'] = randomize(x['item'])

        return decoded_tx

    def assemble_transaction(self, decoded_tx: object) -> str:
        version = decoded_tx['version']
        marker = decoded_tx['marker']
        flag = decoded_tx['flag']
        locktime = decoded_tx['locktime']

        inputs = []
        for tx_in in decoded_tx['tx_in']:
            inputs.append(''.join(str(value) for value in tx_in.values()))

        outputs = []
        for tx_out in decoded_tx['tx_out']:
            inputs.append(''.join(str(value) for value in tx_out.values()))

        witnesses = []
        for witness in decoded_tx['witness']:
            inputs.append(''.join(str(value) for value in witness.values()))

        # [version] [flags] [inputs lenght] [inputs] [outputs lenght] [outputs] [witness] [locktime]
        transaction = ''.join([
            version,
            marker,
            flag,
            len(inputs).to_bytes().hex(),
            ''.join(inputs),
            len(outputs).to_bytes().hex(),
            ''.join(outputs),
            len(witnesses).to_bytes().hex(),
            ''.join(witnesses),
            locktime
        ])
        return transaction
