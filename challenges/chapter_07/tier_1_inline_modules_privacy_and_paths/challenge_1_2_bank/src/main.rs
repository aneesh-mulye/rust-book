pub mod account {
    pub struct Account {
        pub owner: String,
        pub id: u32,
        balance: f64,
    }

    impl Account {
        pub fn new(owner: &str, id: u32, initial_deposit: f64) -> Option<Account> {
            if initial_deposit < 0.0 {
                None
            } else {
                Some(Account {
                    owner: String::from(owner),
                    id,
                    balance: initial_deposit,
                })
            }
        }

        pub fn balance(&self) -> f64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: f64) -> bool {
            if amount < 0.0 {
                false
            } else {
                self.balance += amount;
                true
            }
        }

        pub fn withdraw(&mut self, amount: f64) -> bool {
            if amount < 0.0 || amount > self.balance {
                false
            } else {
                self.balance -= amount;
                true
            }
        }
    }
}

pub mod transaction {
    use crate::account;

    pub enum Transaction {
        Deposit(f64),
        Withdrawal(f64),
        Transfer { amount: f64, to_id: u32 },
    }

    pub fn describe(tx: &Transaction) -> String {
        match tx {
            Transaction::Deposit(d) => format!("Deposit {}", d),
            Transaction::Withdrawal(w) => format!("Withdraw {}", w),
            Transaction::Transfer { amount, to_id } => {
                format!("Transfer {} to account id {}", amount, to_id)
            }
        }
    }

    pub fn apply(account: &mut account::Account, tx: &Transaction) -> bool {
        match tx {
            Transaction::Deposit(d) => account.deposit(*d),
            Transaction::Withdrawal(w) => account.withdraw(*w),
            Transaction::Transfer { amount, to_id: _ } => account.withdraw(*amount),
        }
    }
}

fn main() {
    let mut primary = account::Account::new("Alice", 1, 100.0)
        .expect("primary account should be created with a non-negative deposit");
    let savings = account::Account::new("Bob", 2, 40.0)
        .expect("secondary account should be created with a non-negative deposit");

    let transactions = [
        transaction::Transaction::Deposit(50.0),
        transaction::Transaction::Withdrawal(25.0),
        transaction::Transaction::Transfer {
            amount: 30.0,
            to_id: savings.id,
        },
        transaction::Transaction::Withdrawal(500.0),
        transaction::Transaction::Deposit(-10.0),
    ];

    for tx in &transactions {
        let description = transaction::describe(tx);
        let success = transaction::apply(&mut primary, tx);
        println!("{description} -> success: {success}");
    }

    println!("Final balance: {}", primary.balance());
}

#[cfg(test)]
mod tests {
    use super::account::Account;
    use super::transaction::{self, Transaction};

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn constructor_rejects_negative_initial_deposit() {
        let account = Account::new("Alice", 1, -0.01);
        assert!(
            account.is_none(),
            "Account::new should return None when the initial deposit is negative."
        );
    }

    #[test]
    fn constructor_sets_public_fields_and_private_balance() {
        let account = Account::new("Alice", 7, 125.5)
            .expect("expected account construction to succeed for non-negative deposit");

        assert_eq!(account.owner, "Alice", "Owner should be copied into the account.");
        assert_eq!(account.id, 7, "Account id should be set from the constructor argument.");
        assert!(
            approx_eq(account.balance(), 125.5),
            "balance() should expose the private balance field without changing it."
        );
    }

    #[test]
    fn deposit_accepts_non_negative_and_rejects_negative_amounts() {
        let mut account = Account::new("Alice", 1, 10.0)
            .expect("expected account construction to succeed for non-negative deposit");

        assert!(
            account.deposit(15.0),
            "deposit should succeed for a positive amount."
        );
        assert!(
            approx_eq(account.balance(), 25.0),
            "Successful deposit should increase balance to 25.0."
        );

        assert!(
            !account.deposit(-5.0),
            "deposit should return false for a negative amount."
        );
        assert!(
            approx_eq(account.balance(), 25.0),
            "Rejected deposit should leave the balance unchanged."
        );
    }

    #[test]
    fn withdraw_rejects_negative_amounts_and_overdrafts() {
        let mut account = Account::new("Alice", 1, 30.0)
            .expect("expected account construction to succeed for non-negative deposit");

        assert!(
            !account.withdraw(-1.0),
            "withdraw should reject negative amounts."
        );
        assert!(
            approx_eq(account.balance(), 30.0),
            "Rejected negative withdrawal should not change the balance."
        );

        assert!(
            !account.withdraw(31.0),
            "withdraw should reject amounts larger than the balance."
        );
        assert!(
            approx_eq(account.balance(), 30.0),
            "Rejected overdraft should not change the balance."
        );

        assert!(
            account.withdraw(12.5),
            "withdraw should succeed when the amount is within the balance."
        );
        assert!(
            approx_eq(account.balance(), 17.5),
            "Successful withdrawal should reduce the balance to 17.5."
        );
    }

    #[test]
    fn describe_returns_human_readable_text_for_each_transaction_variant() {
        assert_eq!(
            transaction::describe(&Transaction::Deposit(50.0)),
            "Deposit 50",
            "Deposit description should include the deposited amount."
        );
        assert_eq!(
            transaction::describe(&Transaction::Withdrawal(12.5)),
            "Withdraw 12.5",
            "Withdrawal description should include the withdrawn amount."
        );
        assert_eq!(
            transaction::describe(&Transaction::Transfer {
                amount: 20.0,
                to_id: 9
            }),
            "Transfer 20 to account id 9",
            "Transfer description should include both amount and target account id."
        );
    }

    #[test]
    fn apply_uses_deposit_withdraw_and_withdraw_only_transfer_logic() {
        let mut account = Account::new("Alice", 1, 100.0)
            .expect("expected account construction to succeed for non-negative deposit");

        assert!(
            transaction::apply(&mut account, &Transaction::Deposit(25.0)),
            "Applying a deposit transaction should succeed."
        );
        assert!(
            approx_eq(account.balance(), 125.0),
            "Deposit transaction should increase balance to 125.0."
        );

        assert!(
            transaction::apply(&mut account, &Transaction::Withdrawal(20.0)),
            "Applying a withdrawal transaction should succeed when funds exist."
        );
        assert!(
            approx_eq(account.balance(), 105.0),
            "Withdrawal transaction should reduce balance to 105.0."
        );

        assert!(
            transaction::apply(
                &mut account,
                &Transaction::Transfer {
                    amount: 5.0,
                    to_id: 99,
                },
            ),
            "Transfer should process the withdrawal side when funds exist."
        );
        assert!(
            approx_eq(account.balance(), 100.0),
            "Transfer should only withdraw from the source account in this challenge."
        );
    }

    #[test]
    fn failed_transactions_leave_balance_unchanged() {
        let mut account = Account::new("Alice", 1, 40.0)
            .expect("expected account construction to succeed for non-negative deposit");

        let failed_deposit = transaction::apply(&mut account, &Transaction::Deposit(-5.0));
        assert!(
            !failed_deposit,
            "Negative deposit transaction should fail through account.deposit."
        );
        assert!(
            approx_eq(account.balance(), 40.0),
            "Failed deposit transaction should not change the balance."
        );

        let failed_transfer = transaction::apply(
            &mut account,
            &Transaction::Transfer {
                amount: 100.0,
                to_id: 2,
            },
        );
        assert!(
            !failed_transfer,
            "Transfer larger than the balance should fail because it uses withdraw internally."
        );
        assert!(
            approx_eq(account.balance(), 40.0),
            "Failed transfer should not change the balance."
        );
    }
}
