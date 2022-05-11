#!/bin/bash
##
## This script will build the faucet contract deploy it to dev account and
## export its address to FAUCET_CONTRACT
############
# build the contract
echo -e "-------------------------\nCargo Build - START\n-------------------------"
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
echo -e "-------------------------\nCargo Build - FINISHED\n-------------------------"
echo -e "-------------------------\nDEV-DEPLOY - START\n-------------------------"
# TODO: FIX make sure the output fom dev-deploy is not lost 
faucet_account=$(near dev-deploy ./target/wasm32-unknown-unknown/release/near_testnet_faucet.wasm | grep -i done | awk '{print $4}')
echo -e "-------------------------\nDEV-DEPLOY - DONE\n#########################"
echo -e "# `printf %-5s``tput smul`You should set FAUCET_CONTRACT variable by the following command`tput rmul`"
echo -e "# `printf %-5s``tput bold`export FAUCET_CONTRACT=$faucet_account`tput sgr0`"
echo "#########################"
exit 0
