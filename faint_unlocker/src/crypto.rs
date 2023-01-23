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
        let file_content = fs::read(file).unwrap();
        let cipher = libaes::Cipher::new_256(customer.keypair.key.as_bytes().try_into().unwrap());
        let decrypted = cipher.cbc_decrypt(customer.keypair.iv.as_bytes(), &file_content);
        fs::write(file, decrypted).unwrap();
        fs::rename(
            file,
            file.to_string_lossy()
                .split_at(file.to_string_lossy().len() - 6)
                .0,
        )
        .unwrap();
    };
}
