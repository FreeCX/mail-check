use std::fmt::Debug;
use std::path::Path;
use std::{fs, io::Write};

use anyhow::Context;
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
    pub show_no_internet_msg: bool,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Manager {
    pub config: Config,
    accounts: Vec<Account>,
}

impl Default for Config {
    fn default() -> Self {
        Config { tcp_timeout_secs: 3, online_wait_secs: 3, retry_count: 3, show_no_internet_msg: true }
    }
}

impl Manager {
    pub fn load<P>(config: P) -> anyhow::Result<Manager>
    where
        P: AsRef<Path> + Debug + Copy,
    {
        let raw = fs::read_to_string(config).with_context(|| format!("Failed to load {config:?} file"))?;
        Ok(toml::from_str(&raw).with_context(|| "Failed to parse config")?)
    }

    pub fn save<P>(&mut self, config: P) -> anyhow::Result<()>
    where
        P: AsRef<Path> + Debug + Copy,
    {
        let data = toml::to_string(&self)?;
        let mut f = fs::File::create(config).with_context(|| format!("Failed to create {config:?} file"))?;
        f.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn add_account<I>(&mut self, login: I, domain: I, port: u16) -> anyhow::Result<()>
    where
        I: Into<String>,
    {
        let login = login.into();
        let entry = Entry::new(consts::APPNAME, &login)?;

        if entry.get_password().is_ok() {
            return Err(anyhow::anyhow!("Account {login} already exists in keyring"));
        }

        let password = rpassword::prompt_password(format!("Enter password for {login}: "))?;
        entry.set_password(&password)?;

        self.accounts.push(Account { login, domain: domain.into(), port });

        Ok(())
    }

    pub fn remove_account(&mut self, login: &String) -> anyhow::Result<()> {
        let index = self
            .accounts
            .binary_search_by(|item| item.login.cmp(login))
            .map_err(|_| anyhow::anyhow!("Login {login} not found in keyring"))?;
        let account = self.accounts.swap_remove(index);

        let entry = Entry::new(consts::APPNAME, &account.login)?;
        let _ = entry.delete_password();

        Ok(())
    }

    pub fn check(&self) -> anyhow::Result<u32> {
        let mut total_unread = 0;
        for account in &self.accounts {
            println!("check {}", account.login);

            let entry = Entry::new(consts::APPNAME, &account.login)?;
            let password = entry.get_password().with_context(|| "Failed to get password from keyring")?;

            let mut mail = Mail::connect(&account.domain, account.port, &account.login, &password)?;
            let count = mail.check()?;
            total_unread += count;

            println!("{} have {count} new message(s)", account.login);
        }
        Ok(total_unread)
    }
}
