use std::error::Error;
use std::io;
use std::vec::Vec;

use csv;

use crate::lib::input_file_struct::TransactionData as TransactionData;

pub fn read_from_stdin(path: &str) -> Result<Vec<TransactionData>, Box<dyn Error>> {
    // Creates a new csv `Reader` from `stdin`
    let mut reader = csv::Reader::from_path(path).unwrap();

    {
        let headers = reader.headers()?;
        assert_eq!(headers, vec!["type","client","tx","amount"]);
    }

    let headers = reader.headers()?;

    println!("Headers: {:?}", headers);

    // `.records` return an iterator of the internal
    // record structure
    let mut res= Vec::new();
    for result in reader.deserialize() {
        let record: TransactionData = result?;

        println!("{:?}", record);
        let rec: TransactionData = TransactionData {
            _type: record._type,
            _client: record._client,
            _tx: record._tx,
            _amount: record._amount
        };
        res.push(rec);
    }

    Ok(res)
}