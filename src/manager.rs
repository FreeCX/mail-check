use std::path::Path;
use std::{fs, io::Write};

use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::consts;
use crate::mail::Mail;

#[derive(Serialize, Deserialize)]
pub struct Account {
    login: String,
    domain: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tcp_timeout_secs: u64,
    pub online_wait_secs: u64,
    pub retry_count: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Manager {
    pub config: Config,
    accounts: Vec<Account>,
}

impl Default for Config {
    fn default() -> Self {
        Config { tcp_timeout_secs: 3, online_wait_secs: 3, retry_count: 3 }
    }
}

impl Manager {
    pub fn load<P>(config: P) -> Manager
    where
        P: AsRef<Path>,
    {
        match fs::read_to_string(config) {
            Ok(raw) => toml::from_str(&raw).unwrap(),
            Err(err) => {
                println!("error: {err}");
                Manager { config: Config::default(), accounts: Vec::new() }
            }
        }
    }

    pub fn save<P>(&mut self, config: P)
    where
        P: AsRef<Path>,
    {
        let data = toml::to_string(&self).unwrap();
        let mut f = fs::File::create(config).unwrap();
        let _ = f.write_all(data.as_bytes());
    }

    // TODO: проверка на существование учётки
    pub fn add_account<I>(&mut self, login: I, domain: I, port: u16)
    where
        I: Into<String>,
    {
        let login = login.into();
        let entry = Entry::new(consts::APPNAME, &login).unwrap();
        let password = rpassword::prompt_password(format!("Enter password for {login}: ")).unwrap();
        entry.set_password(&password).unwrap();
        self.accounts.push(Account { login, domain: domain.into(), port });
    }

    pub fn remove_account(&mut self, login: &String) {
        match self.accounts.binary_search_by(|item| item.login.cmp(login)) {
            Ok(index) => {
                let account = self.accounts.swap_remove(index);
                let entry = Entry::new(consts::APPNAME, &account.login).unwrap();
                let _ = entry.delete_password();
            }
            Err(err) => println!("error: {err}"),
        }
    }

    pub fn check(&self) -> u32 {
        let mut total_unread = 0;
        for account in &self.accounts {
            println!("check {}", account.login);
            let entry = Entry::new(consts::APPNAME, &account.login).unwrap();
            let password = entry.get_password().unwrap();
            let mut mail = Mail::connect(&account.domain, account.port, &account.login, &password);
            let count = mail.check();
            println!("{} have {count} new message(s)", account.login);
            total_unread += count;
        }
        total_unread
    }
}
