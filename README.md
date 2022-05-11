# Near Testnet Faucet - WIP

__This project **is currently a work in progress**__


Near Testnet Faucet is my project submission for for NCD held by [Near University](https://www.near.university). It consists of a Smart Contract witten in Rust, couple of bash scripts to interact with it and a [TailwindCSS](https://tailwindcss.com/) and [AlpineJs](https://alpinejs.dev/) frontend, currently deployed at https://near-faucet.io.   

### Prerequisites

In order to compile and run everything you will need:

* Node and [near-cli](https://github.com/near/near-cli) installed
* Rust and WASM toolchain [detailed steps here](https://www.near-sdk.io/)


## Deployment and Usage

Additional notes on how to deploy this on a live or release system. Explaining the most important branches, what pipelines they trigger and how to update the database (if anything special).

### Further development and research/exploration

- [ ] Add [workspaces-rs](https://github.com/near/workspaces-rs/) integration tests. (currently blocked by: [#110](https://github.com/near/workspaces-rs/issues/110))
- [ ] Improve defensive mechanics
- [ ] Move the frontend to [Yew](https://yew.rs/)
- [ ] Stake percentage of the vault/account balance with a testnet validator  
- [ ] Add ability to request USN (either trough XCC to usdn.testnet or via contract ballance support)
- [ ] Explore the idea to support other FTs
    - maybe airdrop mechanics(via collaborative effort) to some kind of Dev FT/NFT holders


### Useful Links

* [Near University](https://near.university)
* [Near University Discord](https://discord.gg/k4pxafjMWA)
* [Near Docs](https://docs.near.org)
* [Near SDK-RS Docs](https://near-sdk.io)
* [Testnet Blockchain Explorer](https://explorer.testnet.near.org/)