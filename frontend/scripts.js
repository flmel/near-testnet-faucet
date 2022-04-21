const near = new nearApi.Near({
    keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore(),
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org'
})

// create wallet connection
const wallet = new nearApi.WalletConnection(near)

// connect to a NEAR smart contract
const contract = new nearApi.Contract(wallet.account(), 'faucet.flmel.testnet', {
    viewMethods: ['get_top_contributors'],
    changeMethods: ['request_funds', 'contribute']
})

// contribution stats
function contributionStats() {
    async function contributorsTopList() {
        let response = await contract.get_top_contributors()
        return response.map(contribution => ({
            accountId: contribution[0],
            amount: nearApi.utils.format.formatNearAmount(contribution[1])
        }))
    }

    return { contributorsTopList }
}

// authentication
function auth() {
    const accountId = wallet.getAccountId()
    function sign_in() {
        wallet.requestSignIn(
            'faucet.flmel.testnet',
            'Testnet Faucet v1'
        )
    }
    function sign_out() {
        wallet.signOut();
        localStorage.removeItem(`near-api-js:keystore:${accountId}:testnet`)
        accountId.value = wallet.getAccountId()
        this.signed = false
    }
    return {
        signed: wallet.isSignedIn(),
        account_id: accountId,
        sign_in,
        sign_out
    }
}

// faucet
function faucet() {
    async function requestFunds() {
        this.loading = true;
        const receiver = this.receiver_id
        try {
            await contract.request_funds({
                receiver,
                amount: 1
            })
            this.success = {
                status: true,
                message: `Success! You have sent 100Ⓝ to ${receiver}!`
            }
            this.loading = false;
        } catch {
            this.failure = {
                status: true,
                message: ''
            }
        }

    }
    // clearNotifications
    function clearNotifications() {
        this.success.status = false;
        this.failure.status = false;
    }
    // checkAccountExist
    async function checkAccountExist() {
        if (this.receiver_id.length < 8 || this.receiver_id_length > 64) {
            this.accountExist = false
        } else {
            try {
                await near.connection.provider.query({
                    request_type: 'view_account',
                    finality: 'final',
                    account_id: this.receiver_id,
                })
                this.accountExist = true
                this.clearNotifications()
            } catch {
                this.accountExist = false
                this.failure = { status: true, message: 'This account does not exist!' }
            }
        }
    }

    return {
        receiver_id: wallet.getAccountId(),
        accountExist: true,
        loading: false,
        success: { status: false, message: '' },
        failure: { status: false, message: '' },
        requestFunds,
        clearNotifications,
        checkAccountExist,
    }
}