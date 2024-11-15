#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_send_tokens() {
        let mut wallet = TokenWallet::default();

        let user1 = Principal::from_text("user1-principal-id").unwrap();
        let user2 = Principal::from_text("user2-principal-id").unwrap();

    
        wallet.balances.insert(user1, 100);
        wallet.balances.insert(user2, 50);

       
        wallet.send_tokens(user2, 30);

        
        assert_eq!(wallet.get_balance(), 70); 
        assert_eq!(wallet.balances.get(&user2).unwrap(), &80); 
    }

    #[test]
    fn test_receive_tokens() {
        
        let mut wallet = TokenWallet::default();

       
        let user1 = Principal::from_text("user1-principal-id").unwrap();

       
        wallet.balances.insert(user1, 100);

        
        wallet.receive_tokens(50);

       
        assert_eq!(wallet.get_balance(), 150); // User1's balance should be 150
    }

    #[test]
    fn test_send_tokens_insufficient_balance() {
        
        let mut wallet = TokenWallet::default();

       
        let user1 = Principal::from_text("user1-principal-id").unwrap();
        let user2 = Principal::from_text("user2-principal-id").unwrap();

       
        wallet.balances.insert(user1, 10);

        
        wallet.send_tokens(user2, 20);

        // Assert that balances remain unchanged
        assert_eq!(wallet.get_balance(), 10); // User1's balance should still be 10
        assert_eq!(wallet.balances.get(&user2).unwrap_or(&0), &0); // User2's balance should still be 0
    }
}
