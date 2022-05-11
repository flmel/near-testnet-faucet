# Near Testnet Faucet - WIP

__This project **is currently a work in progress**__


Near Testnet Faucet is my project submission for for NCD held by [Near University](https://www.near.university). It consists of a Smart Contract witten in Rust, couple of bash scripts to interact with it and a [TailwindCSS](https://tailwindcss.com/) and [AlpineJs](https://alpinejs.dev/) frontend, currently deployed at https://near-faucet.io. It aims to help developers coming from other blockchains who are used to the concept of *Faucets* and people who for some reason are in need of _Testnet_ Near.


### Prerequisites

In order to compile and run everything you will need:

* Node and [near-cli](https://github.com/near/near-cli) installed
* Rust and WASM toolchain [detailed steps here](https://www.near-sdk.io/)


## Deployment and Usage

Additional notes on how to deploy this on a live or release system. Explaining the most important branches, what pipelines they trigger and how to update the database (if anything special).

build:  
`RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

deploy:  
`near deploy --wasmFile PATH_TO.wasm --accountId ACCOUNT_YOU_HAVE_KEYS_FOR`

Alternatively, you can make use of `near dev deploy` or the included bash script at `./scripts/build-deploy.sh`


#### Brief overview of the contracts functions

```rust 
pub fn request_funds(...) {
// requests funds to be sent to certain receiver_id
}
pub fn contribute(...) {
// records the contributor to the contributors (sorts the Vec before inserting)... 
}
pub fn get_top_contributors(...) {
// retrieves the top ten contributors
}
pub fn add_to_blacklist(...) {
// adds an AccountId to the blacklist
}
pub fn remove_from_blacklist(...) {
// removes an AccountId from the blacklist
} 
pub fn clear_recent_receivers(...) {
// clears the recent_receivers map, thus removing all current time constrains 
}
fn request_additional_liquidity(...) {
// this makes XCC to an vault contract (can be found in vault branch) if the faucets account balance goes bellow certain threshold 
}
```
#### Scripts
A set of bash scripts can be found in `./scripts` these are meant to automate and ease the interaction with the contract. I've tried to document well with comments on each line. Expected to be ran from the main directory. 

NOTES: 
- _Scripts probably wont work on Windows because in Windows we set env variables differently and we have to escape the `"` in the arguments of contract calls when interacting with the cli_  
- _Scripts do not cover all the functionality of the contract_

## Testing
Currently the project makes use of Rusts Unit testing (ish), Integration tests are a bit hard since the tooling is under restructuring/refactoring at the moment.    
test:  
`cargo test `

## Frontend
Frontend consists of a static web app built with [TailwindCSS](https://tailwindcss.com/), [AlpineJs](https://alpinejs.dev/) and [near-api-js](https://github.com/near/near-api-js) which can be found on the [frontend](https://github.com/flmel/near-testnet-faucet/tree/frontend) branch.

### Further development and research/exploration

- [ ] Add [workspaces-rs](https://github.com/near/workspaces-rs/) integration tests. (currently blocked by: [#110](https://github.com/near/workspaces-rs/issues/110))
- [ ] Make the contract emit custom [events](https://nomicon.io/Standards/EventsFormat)
- [ ] Move the frontend to [Yew](https://yew.rs/)
- [ ] Improve defensive mechanics
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