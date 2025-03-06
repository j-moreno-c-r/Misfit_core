#test if the block are valid with just the command createblock with only valid flags like number of txs

#test the flags: 
    # --version Set the block version as random invalid parameter

    # --prevblock Set the previous block hash as random invalid parameter

    # --merkleroot Set the merkle root as random invalid parameter

    # --timestamp Set the timestamp as random invalid parameter

    # --bits Set the bits as random invalid parameter

    # --nonce Set the nonce as random invalid parameter

#now the transaction flags inside the block created previosly:

    #Transaction

        # --tx_version Set the transaction version as random invalid parameter
        
        # --tx_marker Set the transaction market as random invalid parameter

        # --tx_flag Set the transaction flag as random invalid parameter

        # --tx_locktime Set the transaction locktime as random invalid parameter

    #Inputs:
        # --tx_in_count Set the number of transaction inputs
    
        # --invalid_tx_in_count Set the number of invalid transaction inputs (if not set, all inputs will be invalid)

        # --tx_in_txid Set the txid from inputs of transactions as random invalid parameter

        #--tx_in_vout Set the vout from inputs of transactions as random invalid parameter

        #--tx_in_script_size Set the size of scriptsig from inputs of transactions as random invalid parameter

        #--tx_in_script Set the scriptsig from inputs of transactions as random invalid parameter

        #--tx_in_sequence Set the sequence from inputs of transactions as random invalid parameter


    #Outputs:
        # --tx_out_count Set the number of transaction outputs

        # --invalid_tx_out_count Set the number of invalid transaction outputs (if not set, all outputs will be invalid)

        # --tx_out_amount Set the amount from outputs of transactions as random invalid parameter

        # --tx_out_script_size Set the size of scriptpubkey from outputs of transactions as random invalid parameter

        # --tx_out_script Set the scriptpubkey from outputs of transactions as random invalid parameter



    #Witness:
        # --tx_witness_count Set the number of transaction witness itens

        # --invalid_tx_witness_count Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)

        # --tx_witness_size Set the item size from witness item of transactions as random invalid parameter

        # --tx_witness_item Set the item from witness of transactions as random invalid parameter
