#![allow(unused)]
#[derive(Clone, PartialEq)]
pub struct Recipient {
    ip: String,
    alias: Option<String>,
    private_key: Option<Vec<u8>>
}
impl Recipient {
    pub fn set_alias(&mut self, alias: Option<String>) {
        self.alias = alias;
    }
    pub fn full_string(&self) -> String {
        match &self.alias {
            None => self.ip.clone(),
            Some(a) => format!("{} ({})", a, &self.ip)
        }
    }
    pub fn alias(&self) -> Option<String> {
        self.alias.clone()
    }
    pub fn private_key(&self) -> Option<Vec<u8>> {
        self.private_key.clone()
    }
    pub fn set_private_key(&mut self, key: Vec<u8>) {
        self.private_key = Some(key);
    }
    pub fn ip(&self) -> String {
        self.ip.clone()
    }
}

impl From<String> for Recipient {
    fn from(string: String) -> Recipient {
        Recipient {
            ip: string,
            alias: None,
            private_key: None
        }
    }
}
impl From<&str> for Recipient {
    fn from(string: &str) -> Recipient {
        Recipient {
            ip: string.to_string(),
            alias: None,
            private_key: None
        }
    }
}

pub fn is_valid_ip(ip: impl ToString) -> bool {
    let ip = ip.to_string();
    let bytes: Vec<&str> = ip.split_terminator('.').collect();
    if bytes.len() != 4 {return false};
    for byte in bytes.iter() {
        match byte.parse::<u8>() {
            Ok(_) => (),
            Err(_) => return false,
        };
    }
    true
}

pub fn find_alias(ip: impl ToString, find: &Vec<Recipient>) -> Option<String> {
    let ip = ip.to_string();
    for rec in find.iter() {
        if rec.ip() == ip {
            return rec.alias();
        }
    }
    None
}
pub fn modify_alias(ip: impl ToString, alias: Option<String>, find: &mut Vec<Recipient>) {
    let ip = ip.to_string();
    for rec in find.iter_mut() {
        if rec.ip() == ip {
            rec.set_alias(alias.clone());
        }
    }
}

#[derive(Clone, Default)]
pub struct Message {
    author: String,
    content: String
}
impl Message {
    pub fn new(author: String, content: String) -> Self {
        Self {author, content}
    }
    pub fn author(&self) -> String {
        self.author.clone()
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn clean_nulls(&mut self) {
        let mut cleaned = String::new();
        for byte in self.content().bytes() {
            if byte != 0 {
                cleaned.push(byte as char)
            }
        }
        self.content = cleaned;
    }
}

#[derive(Clone)]
pub struct ChatHistory {
    peer: Recipient,
    history: Vec<Message>
}
impl ChatHistory {
    pub fn new(peer: Recipient) -> Self {
        Self {
            peer,
            history: Vec::new()
        }
    }
    pub fn push_msg(&mut self, msg: Message) {
        self.history.push(msg)
    }
    pub fn pop_msg(&mut self) -> Message {
        self.history.pop().unwrap_or_default()
    }
    pub fn peer(&self) -> Recipient {
        self.peer.clone()
    }
    pub fn history(&self) -> Vec<Message> {
        self.history.clone()
    }
    pub fn update_peer(&mut self, new: Recipient) {
        self.peer = new
    }
    pub fn clear_history(&mut self) {
        self.history.clear()
    }
}

pub fn try_refresh_history_list(history_list: &mut Vec<ChatHistory>, peer_list: &Vec<Recipient>, once: bool) {
    for peer in peer_list.iter() {
        let mut matched = false;
        for history in history_list.iter() {
            if &history.peer() == peer {
                matched = true;
                break
            }
        }
        if !matched {
            history_list.push(ChatHistory::new(peer.clone()));
            if once {break}
        }
    }
}