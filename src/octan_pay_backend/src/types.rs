use ic_cdk::export::candid::{CandidType, Deserialize, Nat,};
use ic_cdk::export::Principal;
use std::collections::{HashSet};

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
  pub name: String,
  // pub logo: Option<String>,
  pub symbol: String,
  // pub custodians: Option<HashSet<Principal>>,
  // pub cap: Option<Principal>,
}
#[derive(CandidType, Default)]
pub struct Metadata {
  pub name: String,
  pub symbol: String,
  pub custodians: HashSet<Principal>,
  pub created_at: u64,
  pub upgraded_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Details {
  pub description: String,
  pub meta: String,
}

#[derive(CandidType, Deserialize)]
pub struct Permissions {
  pub can_get: Vec<Principal>,
  pub can_verify: Vec<Principal>,
}

pub type InvoiceId = Nat;

#[derive(CandidType, Deserialize)]
pub struct Invoice {
  pub id : InvoiceId,
  pub from : Principal,
  pub to : Principal,
  pub details : Option<Details>,
  pub permissions : Option<Permissions>,
  pub amount : Nat,
  pub verified_paid_at_time : Option<Nat>,
  pub paid: bool,
}

impl Invoice {
  pub fn new(
    id : InvoiceId, 
    from : Principal,  
    to : Principal,  
    details : Option<Details>,
    permissions : Option<Permissions>,
    amount : Nat,
    verified_paid_at_time : Option<Nat>,
    paid: bool,
  ) -> Self {
    Self {
      id,
      from,
      to,
      details,
      permissions,
      amount,
      verified_paid_at_time,
      paid
    }
  }
}

#[derive(CandidType, Deserialize)]
pub struct AddAllowedCreatorArgs {
  pub who: Principal
}

pub type AddAllowedCreatorResult = Result<AddAllowedCreatorSuccess, AddAllowedCreatorErr>;


pub struct AddAllowedCreatorSuccess {
  pub message: String
}

// pub struct AddAllowedCreatorErr {
//   pub kind: AddAllowedCreatorErrEnum
// }

pub enum AddAllowedCreatorErr {
  AlreadyAdded(u8),
  AnonymousIneligible(u8),
  MaxAllowed(u8),
  NotAuthorized(u8)
}

pub struct RemoveAllowedCreatorArgs {
  pub who: Principal
}

pub type RemoveAllowedCreatorResult = Result<RemoveAllowedCreatorSuccess, RemoveAllowedCreatorErr>;

pub struct  RemoveAllowedCreatorSuccess {
  pub message: String
}

pub enum RemoveAllowedCreatorErr {
  NotAuthorized(u8),
  NotFound(u8)
}

pub type GetAllowedCreatorsListResult = Result<GetAllowedCreatorsListSuccess, GetAllowedCreatorsListErr>;


pub struct GetAllowedCreatorsListSuccess {
  allowed : Vec<Principal>
}

pub enum GetAllowedCreatorsListErr {
  NotAuthorized(u8)
}

#[derive(CandidType, Deserialize)]
pub struct CreateInvoiceArgs {
  pub to: Principal,
  pub amount : Nat,
  pub details : Option<Details>,
  pub permissions : Option<Permissions>
}

pub type CreateInvoiceResult = Result<CreateInvoiceSuccess, CreateInvoiceErr>;

pub struct CreateInvoiceSuccess {
  pub invoice : Invoice
}

pub enum CreateInvoiceErr {
  DescriptionTooLarge(u8),
  InsufficientAmountDue(u8),
  MaxInvoicesCreated(u8),
  MetaTooLarge(u8),
  NotAuthorized(u8),
  TooManyPermissions(u8),
}



#[derive(CandidType, Deserialize)]
pub struct TransferParams {
  pub to: Principal,
  pub amount: Nat,
}