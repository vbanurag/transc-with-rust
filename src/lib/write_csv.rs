use csv::Writer;
use std::error::Error;

use super::accountList::AccountList;

pub fn write_csv(path: &str, data: AccountList) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path).unwrap();

    for (key, value) in data.map.into_iter() {
        wtr.serialize(value)?;
    }

    wtr.flush()?;

    Ok(())
}
