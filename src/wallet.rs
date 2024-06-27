pub struct Wallet {
    uid: String,
    private_key: String,
    balance: i64,
}

impl Wallet {
    pub fn New() -> Result<Wallet, String> {
        Ok(Wallet {
            uid: "".to_string(),
            private_key: "".to_string(),
            balance: 111,
        })
    }
    pub fn balance(&self) -> i64 {
        self.balance
    }
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }
    /// get user wallets number
    pub fn get_user_wallets_number(&self, uid: &str) -> i64 {
        0
    }
}
