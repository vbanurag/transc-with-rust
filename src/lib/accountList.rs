use std::collections::HashMap;

use super::account::Account;

#[derive(Debug, Clone)]
pub struct AccountList {
    pub(crate) map: HashMap<u16, Account>
}

impl AccountList {
    fn insert(&mut self, key: u16, value: Account) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, value);
    }
    pub fn new() -> AccountList {
        AccountList { map: HashMap::new() }
    }
}
