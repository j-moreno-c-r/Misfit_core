#test the valid transaction in all campus to check if its really valid 

#test "--tx_count Set a number of transactions in block"

# --invalid_tx_count Set a number of invalid transactions in block (if is not set, all transactions will be invalid)

    #test the break campus if the campus are realy invalid:
    
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