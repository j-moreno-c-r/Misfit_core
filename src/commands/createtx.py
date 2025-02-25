from argparse import ArgumentParser, Namespace
from src.utils import bcli, randomize
import json
import sys

def register_parser(subparsers) -> ArgumentParser:
    parser = subparsers.add_parser(
        name='createtx',
        help='Create a transaction with invalid specified parameters'
    )
    
    # Transaction
    parser.add_argument('--tx_version', dest="tx_version", action="store_true", help="Set the transaction version as random invalid parameter")
    parser.add_argument('--tx_marker', dest="tx_marker", action="store_true", help="Set the transaction market as random invalid parameter")
    parser.add_argument('--tx_flag', dest="tx_flag", action="store_true", help="Set the transaction flag as random invalid parameter")
    parser.add_argument('--tx_locktime', dest="tx_locktime", action="store_true", help="Set the transaction locktime as random invalid parameter")
    
    # Inputs
    parser.add_argument('--tx_in_count', dest="tx_in_count", default=1, type=int, help="Number of transaction inputs")
    parser.add_argument('--invalid_tx_in_count', dest="invalid_tx_in_count", default=1, type=int, help="Number of invalid transaction inputs")
    parser.add_argument('--tx_in_txid', dest="tx_in_txid", action="store_true", help="Set txid in inputs as random invalid parameter")
    parser.add_argument('--tx_in_vout', dest="tx_in_vout", action="store_true", help="Set vout in inputs as random invalid parameter")
    parser.add_argument('--tx_in_script_size', dest="tx_in_script_size", action="store_true", help="Set size of scriptsig in inputs as random invalid parameter")
    parser.add_argument('--tx_in_script', dest="tx_in_script", action="store_true", help="Set scriptsig in inputs as random invalid parameter")
    parser.add_argument('--tx_in_sequence', dest="tx_in_sequence", action="store_true", help="Set sequence in inputs as random invalid parameter")

    # Outputs
    parser.add_argument('--tx_out_count', dest="tx_out_count", default=1, type=int, help="Number of transaction outputs")
    parser.add_argument('--invalid_tx_out_count', dest="invalid_tx_out_count", default=1, type=int, help="Number of invalid transaction outputs")
    parser.add_argument('--tx_out_amount', dest="tx_out_amount", action="store_true", help="Set amount in outputs as random invalid parameter")
    parser.add_argument('--tx_out_script_size', dest="tx_out_script_size", action="store_true", help="Set size of scriptpubkey in outputs as random invalid parameter")
    parser.add_argument('--tx_out_script', dest="tx_out_script", action="store_true", help="Set scriptpubkey in outputs as random invalid parameter")

    # Witness
    parser.add_argument('--tx_witness_count', dest="tx_witness_count", default=1, type=int, help="Number of transaction witness itens")
    parser.add_argument('--invalid_tx_witness_count', dest="invalid_tx_witness_count", default=1, type=int, help="Number of invalid transaction witness itens")
    parser.add_argument('--tx_witness_size', dest="tx_witness_size", action="store_true", help="Set item size in witness item as random invalid parameter")
    parser.add_argument('--tx_witness_item', dest="tx_witness_item", action="store_true", help="Set item in witness as random invalid parameter")

    return parser

class CreateTx:
    def __init__(self, args: Namespace):
        self.verbose: bool = args.verbose

        # Transaction
        self.tx_version: bool = args.tx_version
        self.tx_marker: bool = args.tx_marker
        self.tx_flag: bool = args.tx_flag
        self.tx_locktime: bool = args.tx_locktime
        
        # Inputs
        self.tx_in_count: int = args.tx_in_count
        self.invalid_tx_in_count: int = args.invalid_tx_in_count or args.tx_in_count
        self.tx_in_txid: bool = args.tx_in_txid
        self.tx_in_vout: bool = args.tx_in_vout
        self.tx_in_script_size: bool = args.tx_in_script_size
        self.tx_in_script: bool = args.tx_in_script
        self.tx_in_sequence: bool = args.tx_in_sequence
        
        # Outputs
        self.tx_out_count: int = args.tx_out_count
        self.invalid_tx_out_count: int = args.invalid_tx_out_count or args.tx_out_count
        self.tx_out_amount: bool = args.tx_out_amount
        self.tx_out_script_size: bool = args.tx_out_script_size
        self.tx_out_script: bool = args.tx_out_script
        
        # Witness
        self.tx_witness_count: int = args.tx_witness_count
        self.invalid_tx_witness_count: int = args.invalid_tx_witness_count or args.tx_witness_count
        self.tx_witness_size: bool = args.tx_witness_size
        self.tx_witness_item: bool = args.tx_witness_item

    def handle(self) -> str:
        # Create a valid raw transaction
        raw_tx = self.create_valid_tx()
        # Decode raw transaction
        decoded_tx = self.split_transaction(raw_tx)
        # Replace decoded raw transaction with misfit parameters
        misfit_tx = self.replace_misfit(decoded_tx)
        # Assemble the misfit transaction
        assembled_tx = self.assemble_transaction(misfit_tx)
        # Return the assebled misfit raw transaction
        return assembled_tx

    def create_valid_tx(self) -> str:
        if self.verbose: print("Creating a valid raw transaction")

        # Check if bitcoind -regtest is running
        if self.verbose: print("Checking if bitcoind is running...")
        try:
            bcli("getblockchaininfo")
        except:
            sys.exit("No nodes detected, please run `bitcoind -regtest`.")

        # Create a wallet
        if self.verbose: print("Trying to create a wallet...")
        try:
            bcli("createwallet misfit-wallet")
        except:
            if self.verbose: print("Wallet already created")
            pass

        # Loads misfit-wallet
        if self.verbose: print("Loading wallet...")
        try:
            bcli("loadwallet misfit-wallet")
        except:
            if self.verbose: print("Wallet already loaded")
            pass

        # Generate new address for wallet
        if self.verbose: print("Generating a new address for wallet...")
        try:
            addr = bcli("getnewaddress misfit-wallet")
        except:
            sys.exit("Failed to generate new address for wallet")

        # Generate funds to the address
        if self.verbose: print("Funding wallet with new blocks...")
        try:
            blocksCount = 101 + self.tx_in_count
            blocksHashs = json.loads(bcli(f"generatetoaddress {str(blocksCount)} {addr}"))
        except:
            sys.exit("Failed to funding wallet")

        # Get block data
        if self.verbose: print("Getting first generated block...")
        firstBlock = json.loads(bcli(f"getblock {blocksHashs[0]} 2"))

        # Create valid raw transaction
        if self.verbose: print("Creating the transaction...")

        inputs = [{
            "txid": firstBlock['tx'][0]['txid'],
            "vout": firstBlock['tx'][0]['vout'][0]['n']
        }]
        for tx_in_count in range(1, self.tx_in_count):
            block = json.loads(bcli(f"getblock {blocksHashs[tx_in_count]} 2"))

            inputs.append({
                "txid": block['tx'][0]['txid'],
                "vout": block['tx'][0]['vout'][0]['n']
            })

        tx_in = json.dumps(inputs, separators=(',', ':'))

        amount = firstBlock['tx'][0]['vout'][0]['value'] - 0.01

        outputs = [{
            bcli("getnewaddress"): amount
        }]

        for tx_out_count in range(1, self.tx_out_count):
            block = json.loads(bcli(f"getblock {blocksHashs[tx_out_count]} 2"))

            outputs.append({
                bcli("getnewaddress"): block['tx'][0]['vout'][0]['value'] - 0.01
            })
        
        tx_out = json.dumps(outputs, separators=(',', ':'))

        raw_tx = bcli(f'createrawtransaction {tx_in} {tx_out}')

        # Sign transaction
        if self.verbose: print("Signing the transaction...")
        signed_tx = json.loads(bcli(f'signrawtransactionwithwallet {raw_tx}'))['hex']

        # Check if is valid tx
        if self.verbose: print("Checking if transaction is valid...")
        try:
            json.loads(bcli(f'testmempoolaccept ["{signed_tx}"]'))[0]
        except:
            return self.create_valid_tx()

        return raw_tx

    def split_transaction(self, raw_tx: str) -> object:
        if self.verbose: print("Spliting raw transaction")

        offset = 0
        txns = bytes.fromhex(raw_tx)

        version = txns[offset:offset+4]
        offset += 4

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
            "tx_in_count": tx_in_count.hex(),
            "tx_in": tx_in,
            "tx_out_count": tx_out_count.hex(),
            "tx_out": tx_out,
            "witness": witness,
            "locktime": lock_time.hex()
        }

    def replace_misfit(self, decoded_tx: object) -> object:
        if self.verbose: print("Replace valid raw transactions with misfit parameters")

        # Transaction
        if self.tx_version: decoded_tx['version'] = randomize(decoded_tx['version'])
        if self.tx_marker: decoded_tx['marker'] = randomize(decoded_tx['marker'])
        if self.tx_flag: decoded_tx['marker'] = randomize(decoded_tx['flag'])
        if self.tx_locktime: decoded_tx['locktime'] = randomize(decoded_tx['locktime'])

        # Inputs
        for invalid_tx_in_count in range(self.invalid_tx_in_count):
            tx_in = decoded_tx['tx_in'][invalid_tx_in_count]

            if self.tx_in_txid:
                tx_in['previous_output'] = randomize(tx_in['previous_output'])

            if self.tx_in_script:
                tx_in['signature_script'] = randomize(tx_in['signature_script'])

            if self.tx_in_sequence:
                tx_in['sequence'] = randomize(tx_in['sequence'])

        # Outputs
        for invalid_tx_out_count in range(1, self.invalid_tx_out_count):
            tx_out = decoded_tx['tx_out'][invalid_tx_out_count]

            if self.tx_out_amount:
                tx_out['value'] = randomize(tx_out['value'])

            if self.tx_out_script_size:
                tx_out['pk_script_length'] = randomize(tx_out['pk_script_length'])

            if self.tx_out_script:
                tx_out['pk_script'] = randomize(tx_out['pk_script'])

        # Witness
        for invalid_tx_witness_count in range(1, self.invalid_tx_witness_count):
            witness = decoded_tx['witness'][invalid_tx_witness_count]

            if self.tx_witness_size:
                witness['size'] = randomize(witness['size'])

            if self.tx_witness_item:
                witness['item'] = randomize(witness['item'])

        return decoded_tx

    def assemble_transaction(self, decoded_tx: object) -> str:
        if self.verbose: print("Assembling the misfit transaction")

        version = decoded_tx['version']
        locktime = decoded_tx['locktime']

        inputs = []
        for tx_in in decoded_tx['tx_in']:
            inputs.append(''.join(str(value) for value in tx_in.values()))

        outputs = []
        for tx_out in decoded_tx['tx_out']:
            outputs.append(''.join(str(value) for value in tx_out.values()))

        witnesses = []
        for witness in decoded_tx['witness']:
            witnesses.append(''.join(str(value) for value in witness.values()))

        # [version] [flags] [inputs lenght] [inputs] [outputs lenght] [outputs] [witness] [locktime]
        transaction = ''.join([
            version,
            len(inputs).to_bytes().hex(),
            ''.join(inputs),
            len(outputs).to_bytes().hex(),
            ''.join(outputs),
            len(witnesses).to_bytes().hex(),
            ''.join(witnesses),
            locktime
        ])
        return transaction

def execute(args: Namespace):
    sys.exit(CreateTx(args).handle())