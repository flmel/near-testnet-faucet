#!/bin/bash
##
## This script will request 10 Testnet Near from the faucet to be send to an  
## account we input. The script will exit with error if the faucet contract is 
## not deployed.
##
## NOTE: Consecutive requset to the same account will be rejected until the required
## time gap has passed (set by REQUEST_GAP_LIMITER in the contract (currently 1 hour)
############
# exit if faucet contract is not deployed
if [ -z ${FAUCET_CONTRACT+x} ]; then echo "You have to run `tput smul`build-deploy`tput rmul` script `tput smul`first`tput rmul`!" && exit 1; fi
# ask for account to send tokens to
echo "Enter the account you want to receive 10 `tput smul`testnet`tput rmul` Near to:"
read account
# make a request for the given account to be sent 10 near in yocto (10*10^24)
near call $FAUCET_CONTRACT request_withraw '{"receiver_id": '\"${account}\"', "10000000000000000000000000"}' --accountId $FAUCET_CONTRACT
exit 0