extern crate clap;

mod lib;

use std::{cell::RefCell, env, rc::Rc};

// use clap::Parser;

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Cli {
//     #[clap(last = true)]
//     f: Vec<String>,
// }

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input = &args[0];
    let output = &args[1];

    let data= lib::read_csv::read_from_stdin(input).unwrap();
    let mut _map = lib::accountList::AccountList::new();
    for chunk in data.chunks(4) {

        for record in chunk {
            if !_map.map.contains_key(&record._client) {
                let _tran = lib::account::Account::new(record._client, 0.0,0.0,0.0, false);
                _map.map.insert(_tran.get_client(), _tran);
            }

            if record._type == "deposit" {
                let _da = _map.map.get_mut(&record._client).unwrap();
                _da.set_total(record._amount);

            } else if record._type == "withdrawal" {
                let _da = _map.map.get_mut(&record._client).unwrap();
                if _da.get_total() >= record._amount {
                    _da.set_total(_da.get_total()-record._amount);
                }
            } else if record._type == "dispute" {
                let _da = _map.map.get_mut(&record._client).unwrap();
                // fetch tranction id data
                let is_exist = data.iter().position(|x| x._tx == record._tx);
                let key = is_exist.is_some();
                let idx = is_exist.unwrap();
                if key {
                    let trasnc_data = data.get(idx).unwrap();
                    _da.set_held(_da.get_held()+trasnc_data._amount);
                    _da.set_available(_da.get_available()-trasnc_data._amount);
                }
            } else if record._type == "resolve" {
                let _da = _map.map.get_mut(&record._client).unwrap();
                let _is_exist = data.iter().position(|x| x._tx == record._tx && x._type == "dispute");
                let _key = _is_exist.is_some();
                let _idx = _is_exist.unwrap();
                if _key {
                    let _trasnc_data = data.get(_idx).unwrap();
                    _da.set_held(_da.get_held()-_trasnc_data._amount);
                    _da.set_available(_da.get_available()+_trasnc_data._amount);
                }
            } else if record._type == "chargeback" {
                let _da = _map.map.get_mut(&record._client).unwrap();
                let _is_exist = data.iter().position(|x| x._tx == record._tx &&  x._type == "dispute");
                let _key = _is_exist.is_some();
                let _idx = _is_exist.unwrap();
                if _key {
                    let _trasnc_data = data.get(_idx).unwrap();
                    _da.set_held(_da.get_held()-_trasnc_data._amount);
                    _da.set_available(_da.get_available()+_trasnc_data._amount);
                    _da.set_locked();
                }
            }
        }
    }
    let dataSet = Rc::new(RefCell::new(_map)).borrow().to_owned();
    lib::write_csv::write_csv(output,  dataSet);
}
