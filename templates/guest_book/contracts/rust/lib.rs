use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Balance, near_bindgen};
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U128};

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
  pub premium: bool, 
  pub sender: AccountId,
  pub text: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct GuestBook {
  messages: Vector<PostedMessage>,
}

impl Default for GuestBook{
  fn default() -> Self {
    Self{messages: Vector::new(b"m")}
  }
}

#[near_bindgen]
impl GuestBook {

  // Public - Adds a new message.
  #[payable]
  pub fn add_message(&mut self, text: String) {
    // If the user attaches more than 0.01N the message is premium
    let premium = env::attached_deposit() >= POINT_ONE;
    let sender = env::predecessor_account_id();

    let message = PostedMessage{premium, sender, text};
    self.messages.push(&message);
  }

  // Returns an array of messages.
  pub fn get_messages(&self, from_index:Option<U128>, limit:Option<u64>) -> Vec<PostedMessage>{
    let from = u128::from(from_index.unwrap_or(U128(0)));

    self.messages.iter()
    .skip(from as usize)
    .take(limit.unwrap_or(10) as usize)
    .collect()
  }

  pub fn total_messages(&self) -> u64 { self.messages.len() }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_message() {
    let mut contract = GuestBook::default();
    contract.add_message("A message".to_string());

    let posted_message = &contract.get_messages(None, None)[0];
    assert_eq!(posted_message.premium, false);
    assert_eq!(posted_message.text, "A message".to_string());
  }

  #[test]
  fn iters_messages() {
    let mut contract = GuestBook::default();
    contract.add_message("1st message".to_string());
    contract.add_message("2nd message".to_string());
    contract.add_message("3rd message".to_string());
    
    let total = &contract.total_messages();
    assert!(*total == 3);

    let last_message = &contract.get_messages(Some(U128::from(1)), Some(2))[1];
    assert_eq!(last_message.premium, false);
    assert_eq!(last_message.text, "3rd message".to_string());
  }
}