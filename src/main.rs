use std::collections::HashMap;
use std::io::{self, Write};

struct BankAccount {
    balance: f64,
}

impl BankAccount {

    fn new() -> BankAccount {
        BankAccount { balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        if 0.0 < amount {
            self.balance += amount;
            println!("Deposited {}. Current Balance {}", amount, self.balance);
        }else {
            println!("Cannot deposite negative amount");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdraw {}. Current Balance {}", amount, self.balance);
        }else {
            println!("Insufficient fund for withdrawal.");
        }
    }

    fn check_balance(&self) -> f64 {
        println!("Your current balance is {}.", self.balance);
        self.balance
    }
}

fn main(){
    let mut account: HashMap<String, BankAccount> = HashMap::new();

    loop {
        println!("Select an option:");

        println!("1. Create account");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Check Balance");
        println!("5. Exit");

        println!("Enter your choice");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                println!("Enter your username for new account: ");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();

                if !account.contains_key(&username) {
                    account.insert(username.clone(), BankAccount::new());
                    println!("Account for '{}' created successfully.", username);
                }else{
                    println!("An account with that username already exists.");
                }
            }

            2 => {
                println!("Enter the username:");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();

                if let Some(account) = account.get_mut(&username) {
                    println!("Enter the deposit amount");
                    io::stdout().flush().unwrap();
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).unwrap();
                    let amount: f64 = amount.trim().parse().unwrap_or(0.0);

                    account.deposit(amount);
                }else {
                    println!("Account not found");
                }
            }

            3 => {
                println!("Enter the username:");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();
                
                if let Some(account) = account.get_mut(&username){
                    println!("Enter amount to withdraw");
                    io::stdout().flush().unwrap();
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).unwrap();
                    let amount = amount.trim().parse().unwrap_or(0.0);

                    account.withdraw(amount);
                }else{
                    println!("Account not found");
                }
            }

            4 => {
                println!("Enter your username");
                io::stdout().flush().unwrap();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                let username = username.trim().to_string();

                if let Some(account) = account.get_mut(&username){
                    account.check_balance();
                }else{
                    println!("Account not found.");
                }
            }

            5 => {
                println!("Exiting program...");
                break;
            }

            _ => {
                println!("Invalid choice. Please try again...");
            }
        }
    }
}