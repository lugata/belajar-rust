fn main() {
    let mut account = BankAccount {
        account_number: "123456789".to_string(),
        balance: 1000.0,
    };
    account.check_balance();

    account.withdraw(50.0);

    account.check_balance();
}

struct BankAccount {
    account_number: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing Rp. {} from account {}",
            amount, self.account_number
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account {} has a balance of Rp. {}",
            self.account_number, self.balance
        );
    }
}
