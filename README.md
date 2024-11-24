# Console Banking Application in Rust

This is a simple console-based application written in Rust. It simulates a banking system where users can create accounts, deposit and withdraw money, check balances, and exit the program.

## Features

1. **Create Account**: Users can create an account by providing a username.
2. **Deposit Money**: Users can deposit money into their account.
3. **Withdraw Money**: Users can withdraw money, provided they have sufficient balance.
4. **Check Balance**: Users can check their current account balance.
5. **Exit**: Users can exit the application.

## How It Works

- **Input/Output**: The application uses the `std::io` module to handle user input (`stdin`) and display output (`stdout`) in the console.
- **Data Storage**: Account data is stored in a `HashMap`, where the keys are usernames (strings) and the values are `BankAccount` structs.
- **Loop for Continuous Interaction**: A `loop` provides a menu-driven interface, allowing users to repeatedly perform operations until they choose to exit.
- **Error Handling**: Basic input validation is implemented, such as handling invalid or empty inputs and preventing negative or excessive withdrawals.

## Running the Application

1. Save the code to a file, e.g., `main.rs`.
2. Compile and run the application using the following commands:

   ```bash
   rustc main.rs
   ./main
   
3. Interact with the program using the options displayed in the console.

## Example Interaction

```bash
Select an option:
1. Create account
2. Deposit Money
3. Withdraw Money
4. Check Balance
5. Exit

Enter your choice:
1
Enter your username for new account:
JohnDoe
Account for 'JohnDoe' created successfully.

Select an option:
2
Enter the username:
JohnDoe
Enter the deposit amount:
500
Deposited 500. Current Balance 500.

Select an option:
4
Enter your username:
JohnDoe
Your current balance is 500.
```

## Dependencies

- Rust Programming Language (no additional crates are required).

## Notes

- This application is a simple demonstration of a banking system in Rust.
- It uses basic concepts such as structs, HashMap, and user input handling.
- Future improvements could include enhanced error handling, persistence (e.g., saving accounts to a file), and better input validation.
