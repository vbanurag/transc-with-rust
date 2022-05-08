use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "client")]
    pub _client: u16,
    #[serde(rename = "tx")]
    pub _tx: u32,
    #[serde(rename = "amount")]
    pub _amount: f32
}

impl std::fmt::Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(type: {}, client: {}, tx: {}, amount: {})", self._type, self._client, self._tx, self._amount)
    }
}