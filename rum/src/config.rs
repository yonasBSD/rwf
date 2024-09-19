use aes::Aes128;
use aes_gcm_siv::{AesGcmSiv, Key};
use once_cell::sync::{Lazy, OnceCell};
use std::io::IsTerminal;
use time::Duration;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub struct Config {
    pub aes_key: Key<AesGcmSiv<Aes128>>, // AES-128 key used for encryption.
    pub cookie_max_age: Duration,
    pub tty: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aes_key: Key::<AesGcmSiv<Aes128>>::default(),
            cookie_max_age: Duration::days(30),
            tty: std::io::stderr().is_terminal(),
        }
    }
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::default())
}