# octan_pay

Welcome to your new octan_pay project and to the internet computer development community. 
Octan_pay is a payment tool written in Rust, help users use ckBTC tokens to pay for minting Octan Soulbound Token, a kind of Web3 Identity on ICP. Visit Octan Soulbound Token https://github.com/Octan-Labs/Octan-SoulBound-ICP  
Visit Octan code demo on Youtube: https://youtu.be/nUUPJ-SRaSo

To learn more before you start working with octan_pay, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd octan_pay/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
