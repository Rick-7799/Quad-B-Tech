use ic_cdk::api;
use ic_cdk::export::Principal;
use std::collections::HashMap;

#[derive(Default)]
pub struct TokenWallet {
    balances: HashMap<Principal, u64>,
}

impl TokenWallet {
    pub fn send_tokens(&mut self, recipient: Principal, amount: u64) {
        let sender = api::caller();
        let sender_balance = self.balances.entry(sender).or_insert(0);
        
        if *sender_balance >= amount {
            *sender_balance -= amount;
            let recipient_balance = self.balances.entry(recipient).or_insert(0);
            *recipient_balance += amount;
        }
    }

    pub fn receive_tokens(&mut self, amount: u64) {
        let sender = api::caller();
        let sender_balance = self.balances.entry(sender).or_insert(0);
        *sender_balance += amount;
    }

    pub fn get_balance(&self) -> u64 {
        let caller = api::caller();
        *self.balances.get(&caller).unwrap_or(&0)
    }
}
