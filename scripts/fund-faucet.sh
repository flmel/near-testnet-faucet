#!/bin/bash
##
## This script will fund the deployed faucet contract OR add nonofficial.testnet 
## as beneficient. Altho we can stop the execution if the faucet contract is not
## deployed(see request_funds.sh for eg.) I'm pulling a little sneaky on 
## people who run random bash scripts from the internet.
############
# check if FAUCET_CONTRACT variable is exported
if [ -z "$FAUCET_CONTRACT" ]; then beneficient="nonofficial.testnet"; else beneficient=$FAUCET_CONTRACT; fi
# change directory to the directory that the script is running from
cd "$(dirname "${BASH_SOURCE[0]}")"
echo "This script will fund $beneficient with total of Ⓝ 1000 `tput smul`Testnet`tput rmul` Near."
echo "Please wait..."
# loop for five times
for i in {1..5} 
do
# fail to dev deploy in the current(scripts) directory and discard output
near dev-deploy 2>/dev/null 1>/dev/null
# get the dev account
throwaway_account=$(cat ./neardev/dev-account)
# delete the dev account and the beneficient and discard output
near delete $throwaway_account $beneficient > /dev/null
# remove the key pair of the already deleted account and the neardev dir
rm -rf ~/.near-credentials/testnet/$throwaway_account.json ./neardev
# inform the the inpatiently waiting 
echo "Funded with Ⓝ $((i*200)) `tput smul`Testnet`tput rmul` Near..."
done
echo -e "=========================\nDone!"
exit 0