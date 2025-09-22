use arboard::Clipboard;
use rand::{Rng, rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::{env, fs, io, process::exit, thread, time::Duration};
use tabled::{Table, Tabled};

#[derive(Debug, Serialize, Deserialize, Tabled)]
struct AccountEntry {
    account: String,
    username: String,
    email: String,
    #[tabled(skip)]
    password: String,
}

fn main() {
    let database_path = "./database.json";
    if fs::read_to_string(database_path).is_err() {
        save_database_file(database_path, vec![], "");
    }

    let args = env::args();

    if args.len() < 2 {
        eprintln!("Please provide a valid command.");
        exit(1)
    }

    let mut user_command = String::from("");

    for (index, argument) in args.enumerate() {
        if index == 1 {
            user_command = argument;
            break;
        }
    }

    match user_command.to_lowercase().as_str() {
        "help" => show_help(),
        "add" => add_account(database_path),
        "list" => list_accounts(database_path),
        "get" => get_account(database_path),
        "delete" => delete_account(database_path),
        not_found => eprintln!(
            "Unknown command: {}. Use 'help' to see available commands.",
            not_found
        ),
    }
}

// Commands functions

fn show_help() {
    println!("Password Manager CLI");
    println!("Usage:");
    println!("\tpassman <user_command> [options]");
    println!("Commands:");
    println!("\tadd <account> <username>    Add a new password (will prompt for password)");
    println!("\tlist                        Show all saved accounts");
    println!("\tget <account>               Show or copy the password for an account");
    println!("\tdelete <account>            Remove an account");
    println!("\thelp                        Show this help message");
}

fn add_account(database_path: &str) {
    let mut accounts = load_database(database_path);

    let account = prompt_user_input("Enter account:");
    let username = prompt_user_input("Enter username:");
    let email = prompt_user_input("Enter email:");

    let password = match prompt_yes_no("Generate password? [y/n]:") {
        true => generate_random_password(),
        false => prompt_user_input("Please enter your password:"),
    };

    let entry = AccountEntry {
        account,
        username,
        email,
        password,
    };

    accounts.push(entry);

    save_database_file(database_path, accounts, "Account added successfully!");
}

fn list_accounts(database_path: &str) {
    let accounts = load_database(database_path);
    if !accounts.is_empty() {
        let table = Table::new(accounts);
        println!("{table}");
    } else {
        println!("No accounts found in the database.");
    }
}

fn get_account(database_path: &str) {
    let account_name = prompt_user_input("Enter your account's name:");

    let accounts = load_database(database_path);

    let mut is_found = false;
    for entry in accounts {
        if entry.account.to_lowercase() == account_name.to_lowercase() {
            let table = Table::new(vec![&entry]);
            println!("{table}");

            match prompt_yes_no("Do you want to copy the password to clipboard? [y/n]") {
                true => copy_to_clipboard(entry.password),
                false => println!(
                    "Password for account '{}': {}",
                    entry.account, entry.password
                ),
            }

            is_found = true;
            break;
        }
    }

    if !is_found {
        println!("No account found with name '{}'", account_name)
    }
}

fn delete_account(database_path: &str) {
    let mut accounts = load_database(database_path);
    if accounts.is_empty() {
        println!("No accounts available to delete.");
        exit(0);
    }
    list_accounts(database_path);
    let account_to_delete = prompt_user_input("Which account you want to delete?");
    let mut is_found = false;
    let mut delete_index = 0;
    for (index, entry) in accounts.iter().enumerate() {
        if entry.account.to_lowercase() == account_to_delete.to_lowercase() {
            delete_index = index;
            is_found = true;
            break;
        }
    }
    if is_found {
        accounts.remove(delete_index);
        save_database_file(database_path, accounts, "Account deleted successfully!");
    } else {
        println!("No account found with name '{}'", account_to_delete);
    }
}
// Utility functions

fn prompt_user_input(prompt: &str) -> String {
    if !prompt.is_empty() {
        println!("{prompt}");
    }
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    input_buffer.trim().to_string()
}

fn prompt_yes_no(prompt_message: &str) -> bool {
    let user_choice = prompt_user_input(prompt_message);
    matches!(user_choice.trim().to_lowercase().as_str(), "y")
}

fn generate_random_password() -> String {
    let password_length = 20;

    let mut password = String::new();

    password.push(get_random_lowercase());
    password.push(get_random_uppercase());
    password.push(get_random_number());
    password.push(get_random_symbol());

    while password.len() < password_length {
        let random_char_type = rand::rng().random_range(0..4);
        match random_char_type {
            0 => password.push(get_random_lowercase()),
            1 => password.push(get_random_uppercase()),
            2 => password.push(get_random_number()),
            _ => password.push(get_random_symbol()),
        }
    }

    let mut password_vec: Vec<char> = password.chars().collect();
    password_vec.shuffle(&mut rng());
    password = password_vec.iter().collect();
    password
}

fn get_random_lowercase() -> char {
    let pool_lowercase = "abcdefghijklmnopqrstuvwxyz";
    pool_lowercase
        .chars()
        .nth(rand::rng().random_range(0..pool_lowercase.len()))
        .unwrap()
}

fn get_random_uppercase() -> char {
    let pool_uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pool_uppercase
        .chars()
        .nth(rand::rng().random_range(0..pool_uppercase.len()))
        .unwrap()
}

fn get_random_number() -> char {
    let pool_numbers = "0123456789";
    pool_numbers
        .chars()
        .nth(rand::rng().random_range(0..pool_numbers.len()))
        .unwrap()
}

fn get_random_symbol() -> char {
    let pool_symbols = "!@#$%^&*()-_=+[]{};:,.<>?";
    pool_symbols
        .chars()
        .nth(rand::rng().random_range(0..pool_symbols.len()))
        .unwrap()
}

fn load_database(database_path: &str) -> Vec<AccountEntry> {
    let database_contents = fs::read_to_string(database_path).unwrap();
    let accounts: Vec<AccountEntry> = serde_json::from_str(database_contents.as_str()).unwrap();
    accounts
}

fn copy_to_clipboard(password_text: String) {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(password_text).unwrap();
        println!("Password copied to clipboard!");
        thread::sleep(Duration::from_millis(500));
    })
    .join()
    .unwrap();
}

fn save_database_file(database_path: &str, accounts: Vec<AccountEntry>, prompt_message: &str) {
    let accounts_json = serde_json::to_string(&accounts).unwrap();
    fs::write(database_path, accounts_json).unwrap();
    if !prompt_message.is_empty() {
        println!("{prompt_message}");
    }
}
