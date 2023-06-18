#!/bin/bash

# Original source
# https://github.com/dfinity/ckBTC-Minter-Frontend/blob/master/local_deploy.sh

ICRC1_CANISTER="icrc1_token_ledger_canister_ex1"

dfx canister create "$ICRC1_CANISTER" --network "$DFX_NETWORK"
# dfx canister create ckbtc_minter --network "$DFX_NETWORK"
# dfx canister create ckbtc_kyt --network "$DFX_NETWORK"
# dfx canister create ckbtc_index --network "$DFX_NETWORK"

# MINTERID="$(dfx canister id ckbtc_minter --network "$DFX_NETWORK")"
# echo "$MINTERID"
# LEDGERID="$(dfx canister id ckbtc_ledger --network "$DFX_NETWORK")"
# echo "$LEDGERID"
# KYTID="$(dfx canister id ckbtc_kyt --network "$DFX_NETWORK")"
# echo "$KYTID"

CURRENT_IDENTITY_PRINCIPAL="$(dfx identity get-principal --network "$DFX_NETWORK")"

dfx deploy "$ICRC1_CANISTER" --argument '( 
  record { 
    token_symbol = "ICRC1TEST";
    token_name =  "Icrc1Test";
    minting_account = record { owner = principal "'${CURRENT_IDENTITY_PRINCIPAL}'" };
    transfer_fee = 10_000;
    metadata = vec {};
    initial_balances = vec { }; 
    archive_options = record {
      num_blocks_to_archive = 2000;
      trigger_threshold = 1000;
      controller_id = principal "'${CURRENT_IDENTITY_PRINCIPAL}'";
    };
  }
)'

