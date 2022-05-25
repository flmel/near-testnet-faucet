const near = new nearApi.Near({
    keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore(),
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org'
})

// create wallet connection
const wallet = new nearApi.WalletConnection(near)

// connect to a NEAR smart contract
const contract = new nearApi.Contract(wallet.account(), 'v1.faucet.nonofficial.testnet', {
    viewMethods: ['get_top_contributors'],
    changeMethods: ['request_funds', 'contribute']
})

// AlpineJS store for global events/data
document.addEventListener('alpine:init', () => {
    const accountId = wallet.getAccountId()
    Alpine.store('auth', {
        sign_in() {
            wallet.requestSignIn(
                'v1.faucet.nonofficial.testnet',
                'Testnet Faucet v1'
            )
        },
        sign_out() {
            wallet.signOut()
            localStorage.removeItem(`near-api-js:keystore:${accountId}:testnet`)
            accountId.value = accountId
            this.signed = false
        },

        signed: wallet.isSignedIn()
    })
})

// contribute form
function contribute() {
    async function get_balance() {
        const response = await wallet.account().getAccountBalance()
        return nearApi.utils.format.formatNearAmount(response.available).split(".")[0]
    }

    async function contributeToFaucet() {
        await contract.contribute({}, "60000000000000",
            nearApi.utils.format.parseNearAmount(this.contribute_amount).toString())
    }

    function validateInput() {
        this.inputValid = this.contribute_amount !== "" && /^\d*$/.test(this.contribute_amount) && this.contribute_amount != 0
    }

    return {
        get_balance,
        contributeToFaucet,
        validateInput,
        inputValid: true,
        open: false,
        contribute_amount: "200"
    }
}

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

// faucet
function faucet() {
    async function requestFunds() {
        this.loading = true
        const receiver_id = this.receiver_id
        try {
            await contract.request_funds({
                receiver_id,
                amount: nearApi.utils.format.parseNearAmount((100).toString())
            }, "50000000000000")
            this.success = {
                status: true,
                message: `Success! You have sent 100Ⓝ to ${receiver_id}!`
            }
            this.loading = false
        } catch (err) {
            console.error(err)
            this.failure = {
                status: true,
                message: 'Sorry! You have to wait a little longer!'
            }
            this.loading = false
        }

    }
    // clearNotifications
    function clearNotifications() {
        this.success.status = false
        this.failure.status = false
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
