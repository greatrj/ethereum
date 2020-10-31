#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::all, clippy::pedantic)]

extern crate alloc;

mod account;
mod block;
mod header;
mod log;
mod receipt;
mod transaction;

/// This type alias is necessary for rlp-derive to work correctly
pub type Bytes = alloc::vec::Vec<u8>;

pub use account::Account;
pub use block::Block;
pub use header::{Header, PartialHeader};
pub use log::Log;
pub use receipt::Receipt;
pub use transaction::{Transaction, TransactionAction, TransactionSignature};
