use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool
}

impl Copy for Account { }

impl Account {
    pub fn get_client(self) -> u16 {
        self.client
    }
    pub fn get_total(self) -> f32 {
        self.total
    }
    pub fn set_total(&mut self, amt: f32) {
        self.total += amt;
    }
    pub fn get_held(self) -> f32 {
        self.held
    }
    pub fn set_held(&mut self, amt: f32) {
        self.held += amt;
    }
    pub fn get_available(self) -> f32 {
        self.available
    }
    pub fn set_available(&mut self, amt: f32) {
        self.available += amt;
    }
    pub fn set_locked(&mut self) {
        self.locked = false;
    }
    pub fn get_locked(self) -> bool {
        self.locked
    }
    pub fn new(client: u16, available: f32, held: f32, total: f32, locked: bool) -> Account {
        Account {client: client, available: available, held: held, total: total, locked: locked}
    }
    pub fn clone(&self) -> Account {
        *self
    }
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(id: {}, amount: {})", self.client, self.total)
    }
}
