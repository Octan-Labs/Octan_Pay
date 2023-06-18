use ic_cdk::export::candid::{Nat};
use ic_cdk::export::Principal;
use ic_cdk::api::{id};
use std::cell::RefCell;
use std::collections::{HashMap};

use crate::types::*;

thread_local!(
    static LEDGER: RefCell<Ledger> = RefCell::new(Ledger::default());
);

pub fn with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&ledger.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
}

#[derive(Default)]
pub struct Ledger {
  // pub name: String,
  // pub symbol: String,
  // pub owners: HashMap<TokenIdentifier, Principal>,
  // pub tokens: HashMap<Principal, TokenIdentifier>, 
  pub revoked: HashMap<InvoiceId, Invoice>,
  // pub archives: HashMap<Principal, HashSet<TokenIdentifier>>,
  // pub operators: HashMap<Principal, HashSet<TokenIdentifier>>,
  pub invoice_count: Nat,
  pub allowed_creators_list: Vec<Principal>,
  pub is_already_processing_timeout: Nat
}

impl Ledger {
    pub fn init_metadata(&mut self, default_custodian: Principal) {
      // self.name = args.name;
      // self.symbol = args.symbol;
      // let metadata = self.metadata_mut();
      // metadata.custodians.insert(default_custodian);
      // metadata.created_at = time();
      // metadata.upgraded_at = time();
      // self.is_already_processing_timeout = Nat(1000_000_000_000)  // "10 minutes ns"
    }

    pub fn get_invoice_canister_id(&self)-> Principal { 
      id()
    }


    pub fn create_invoice(&mut self, caller: Principal, args: CreateInvoiceArgs) -> Invoice {
      self.invoice_count += 1;


      let invoice = Invoice::new(
        Nat::from(self.invoice_count.clone()), 
        caller,
        args.to,
        args.details,
        args.permissions,
        args.amount,
        None,
        false,
      );

      invoice
    }
}
