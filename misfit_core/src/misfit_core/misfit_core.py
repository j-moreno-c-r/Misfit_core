import argparse
# from createblock import CreateBlock
from src.createtx import CreateTx

def main():
    parser = argparse.ArgumentParser(
        description="A tool for create specified invalid parameters for tests in bicoin.")
    subparsers = parser.add_subparsers(title="Commands", dest="commands")

    # Commands
    parser_createblock = subparsers.add_parser(
        "createblock", help="Create a block with invalid specified parameters")
    parser_createtx = subparsers.add_parser(
        "createtx", help="Create transaction with invalid specified parameters")

    # createblock arguments
    parser_createblock.add_argument('--version',
                                    action="store_true", help="Set the block version as random invalid parameter")
    parser_createblock.add_argument('--prevblock',
                                    action="store_true", help="Set the previous block hash as random invalid parameter")
    parser_createblock.add_argument('--merkleroot',
                                    action="store_true", help="Set the merkle root as random invalid parameter")
    parser_createblock.add_argument('--timestamp',
                                    action="store_true", help="Set the timestamp as random invalid parameter")
    parser_createblock.add_argument('--bits',
                                    action="store_true", help="Set the bits as random invalid parameter")
    parser_createblock.add_argument('--nonce',
                                    action="store_true", help="Set the nonce as random invalid parameter")
    parser_createblock.add_argument('--tx_count', dest="block_tx_count",
                                    default=0, type=int, help="Set a number of transactions in block")
    parser_createblock.add_argument('--invalid_tx_count', default=0, type=int,
                                    help="Set a number of invalid transactions in block (if is not set, all transactions will be invalid)")
    parser_createblock.add_argument('--tx_version',
                                    action="store_true", help="Set the transaction version as random invalid parameter")
    parser_createblock.add_argument('--tx_marker',
                                    action="store_true", help="Set the transaction market as random invalid parameter")
    parser_createblock.add_argument('--tx_flag',
                                    action="store_true", help="Set the transaction flag as random invalid parameter")
    parser_createblock.add_argument('--tx_locktime',
                                    action="store_true", help="Set the transaction locktime as random invalid parameter")
    parser_createblock.add_argument('--tx_in_count', dest="block_tx_in_count",
                                    default=0, type=int, help="Set the number of transaction inputs")
    parser_createblock.add_argument('--invalid_tx_in_count', default=0,
                                    type=int, help="Set the number of invalid transaction inputs (if not set, all inputs will be invalid)")
    parser_createblock.add_argument('--tx_in_txid',
                                    action="store_true", help="Set the txid from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_vout',
                                    action="store_true", help="Set the vout from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_script_size', default=0,
                                    type=int, help="Set the size of scriptsig from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_script',
                                    action="store_true", help="Set the scriptsig from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_sequence',
                                    action="store_true", help="Set the sequence from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_count', dest="block_tx_out_count",
                                    default=0, type=int, help="Set the number of transaction outputs")
    parser_createblock.add_argument('--invalid_tx_out_count', default=0,
                                    type=int, help="Set the number of invalid transaction outputs (if not set, all outputs will be invalid)")
    parser_createblock.add_argument('--tx_out_amount', default=0,
                                    type=int, help="Set the amount from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_script_size', default=0,
                                    type=int, help="Set the size of scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_script',
                                    action="store_true", help="Set the scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_witness_count', dest="block_tx_witness_count",
                                    default=0, type=int, help="Set the number of transaction witness itens")
    parser_createblock.add_argument('--invalid_tx_witness_count', default=0, type=int,
                                    help="Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)")
    parser_createblock.add_argument('--tx_witness_size', default=0,
                                    type=int, help="Set the item size from witness item of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_witness_item',
                                    action="store_true", help="Set the item from witness of transactions as random invalid parameter")

    # createtx arguments
    parser_createtx.add_argument('--tx_count', default=0,
                                 type=int, help="Set a number of transactions in block")
    parser_createtx.add_argument('--invalid_tx_count', default=0, type=int,
                                 help="Set a number of invalid transactions in block (if is not set, all transactions will be invalid)")
    parser_createtx.add_argument('--tx_version',
                                 action="store_true", help="Set the transaction version as random invalid parameter")
    parser_createtx.add_argument('--tx_marker',
                                 action="store_true", help="Set the transaction market as random invalid parameter")
    parser_createtx.add_argument('--tx_flag',
                                 action="store_true", help="Set the transaction flag as random invalid parameter")
    parser_createtx.add_argument('--tx_locktime',
                                 action="store_true", help="Set the transaction locktime as random invalid parameter")
    parser_createtx.add_argument('--tx_in_count', dest="tx_in_count",
                                 default=0, type=int, help="Set the number of transaction inputs")
    parser_createtx.add_argument('--invalid_tx_in_count', default=0, type=int,
                                 help="Set the number of invalid transaction inputs (if not set, all inputs will be invalid)")
    parser_createtx.add_argument('--tx_in_txid', action="store_true",
                                 help="Set the txid from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_vout', action="store_true",
                                 help="Set the vout from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_script_size', default=0, type=int,
                                 help="Set the size of scriptsig from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_script', action="store_true",
                                 help="Set the scriptsig from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_sequence',
                                 action="store_true", help="Set the sequence from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_count', dest="tx_out_count",
                                 default=0, type=int, help="Set the number of transaction outputs")
    parser_createtx.add_argument('--invalid_tx_out_count', default=0, type=int,
                                 help="Set the number of invalid transaction outputs (if not set, all outputs will be invalid)")
    parser_createtx.add_argument('--tx_out_amount', default=0, type=int,
                                 help="Set the amount from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_script_size', default=0, type=int,
                                 help="Set the size of scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_script', action="store_true",
                                 help="Set the scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_witness_count', dest="tx_witness_count",
                                 default=0, type=int, help="Set the number of transaction witness itens")
    parser_createtx.add_argument('--invalid_tx_witness_count', default=0, type=int,
                                 help="Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)")
    parser_createtx.add_argument('--tx_witness_size', default=0, type=int,
                                 help="Set the item size from witness item of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_witness_item',
                                 action="store_true", help="Set the item from witness of transactions as random invalid parameter")

    args = parser.parse_args()

    if args.commands is None:
        parser.print_help()
    elif args.commands == "createblock":
        # Run createblock
        # createblock()
        # misfit_block = CreateBlock(
        #     **dict(args._get_kwargs())).create_misfit_block()
        # print(misfit_block)
        pass
    elif args.commands == "createtx":
        # Run createtx
        misfit_transaction = CreateTx(
            **dict(args._get_kwargs())).create_misfit_transaction()
        print(misfit_transaction)
    else:
        parser.print_help()
        sys.exit("No command provided.")


if __name__ == "__main__":
    main()
