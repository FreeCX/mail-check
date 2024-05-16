use anyhow::Context;
use imap::{ImapConnection, Session};

pub struct Mail {
    session: Session<Box<dyn ImapConnection>>,
}

impl Mail {
    pub fn connect(domain: &str, port: u16, login: &str, password: &str) -> anyhow::Result<Mail> {
        let client = imap::ClientBuilder::new(domain, port).connect()?;
        let session = client.login(login, password).map_err(|(err, _)| err).with_context(|| "Failed to login in")?;
        Ok(Mail { session })
    }

    pub fn check(&mut self) -> anyhow::Result<u32> {
        Ok(self.session.status("INBOX", "(UNSEEN)").map(|mailbox| mailbox.unseen.unwrap_or(0))?)
    }
}

impl Drop for Mail {
    fn drop(&mut self) {
        let _ = self.session.logout();
    }
}
