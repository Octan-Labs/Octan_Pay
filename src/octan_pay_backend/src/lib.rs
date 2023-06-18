use ic_cdk::call;
use ic_cdk_macros::*;
use ic_cdk::export::candid::{Nat, candid_method};
use ic_cdk::api::{caller};
use ic_cdk::api::call::{ManualReply};
use ic_cdk::export::{Principal};

mod types;
mod ledger;

use types::*;

#[init]
fn init() {
  ledger::with_mut(|ledger| ledger.init_metadata(caller()));
}

#[query( manual_reply = true)]
fn canister_id() -> ManualReply<Principal> {
    ledger::with(|ledger| ManualReply::one(ledger.get_invoice_canister_id()))
}

#[update(manual_reply = true)]
fn create_invoice(args: CreateInvoiceArgs) -> ManualReply<Invoice> {
  ledger::with_mut(|ledger| ManualReply::one(ledger.create_invoice(caller(), args)))
}

// #[update(manual_reply = true)]
// fn verify_invoice(args: VerifyInvoiceArgs) -> ManualReply<Invoice> {
//   ledger::with_mut(|ledger| ManualReply::one(ledger.verify_invoice()))
// }

// #[update(name = "transfer", manual_reply = true)]
// async fn transfer()  -> Result<Nat, TransferError>{
   
// }



// ==================================================================================================
// upgrade
// ==================================================================================================
/// NOTE:
/// If you plan to store gigabytes of state and upgrade the code,
/// Using stable memory as the main storage is a good option to consider
// #[pre_upgrade]
// fn pre_upgrade() {
//     ledger::with(|ledger| {
//         if let Err(err) = ic_cdk::storage::stable_save::<(&ledger::Ledger, cap_sdk::Archive)>((
//             ledger,
//             cap_sdk::archive(),
//         )) {
//             trap(&format!(
//                 "An error occurred when saving to stable memory (pre_upgrade): {:?}",
//                 err
//             ));
//         };
//     })
// }

// #[post_upgrade]
// fn post_upgrade() {
//     ledger::with_mut(|ledger| {
//         match ic_cdk::storage::stable_restore::<(ledger::Ledger, cap_sdk::Archive)>() {
//             Ok((ledger_store, cap_store)) => {
//                 *ledger = ledger_store;
//                 ledger.metadata_mut().upgraded_at = time();
//                 cap_sdk::from_archive(cap_store);
//             }
//             Err(err) => {
//                 trap(&format!(
//                     "An error occurred when loading from stable memory (post_upgrade): {:?}",
//                     err
//                 ));
//             }
//         }
//     })
// }

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    std::print!("{}", export_candid());
}

#[query()]
fn export_candid() -> String {
    ic_cdk::export::candid::export_service!();
    __export_service()
}