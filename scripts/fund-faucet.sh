#!/bin/bash

# This script will fund the deployed contract if exists or flmel.testnet
beneficient="flmel.testnet"

for i in {0..5} 
do
near dev-deploy 2>/dev/null 1>/dev/null
throwaway_account=$(cat ./neardev/dev-account)
near delete $throwaway_account $beneficient > /dev/null
echo "Funded faucet with 200 Testnet Near!"
done
rm -rf ./neardev
echo "Removed neardev folder... Funding Complete"