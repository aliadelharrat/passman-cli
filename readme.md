# passman-cli

A simple, terminal-based password manager written in Rust. Store, retrieve, and manage your account credentials safely.

## Features

- Add new accounts with username, email, and password.
- Generate strong random passwords.
- List all saved accounts in a table (passwords hidden).
- Get account details and optionally copy the password to clipboard.
- Delete accounts you no longer need.

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/aliadelharrat/passman-cli.git
   cd passman-cli
````

3. Build the project:
   ```bash
   cargo build --release
````


## Usage

```bash
# Show help
./target/release/passman-cli help

# Add a new account
./target/release/passman-cli add

# List all accounts
./target/release/passman-cli list

# Get account info
./target/release/passman-cli get

# Delete an account
./target/release/passman-cli delete
```

## License

This project is licensed under the MIT License.
