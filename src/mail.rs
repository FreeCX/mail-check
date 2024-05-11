use imap::{ImapConnection, Session};

pub struct Mail {
    session: Session<Box<dyn ImapConnection>>,
}

impl Mail {
    pub fn connect(domain: &str, port: u16, login: &str, password: &str) -> Mail {
        let client = imap::ClientBuilder::new(domain, port).connect().unwrap();
        let session = client.login(login, password).unwrap();
        Mail { session }
    }

    pub fn check(&mut self) -> u32 {
        self.session.status("INBOX", "(UNSEEN)").map(|mailbox| mailbox.unseen.unwrap_or(0)).unwrap()
    }
}

impl Drop for Mail {
    fn drop(&mut self) {
        self.session.logout().unwrap();
    }
}
