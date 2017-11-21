use serde_json::{Value};
use serde_json;
use std::io;
use std::fs::File;
use std::io::Read;

// pub fn get_config() -> Result<Value, io::Error> {
// 	let mut file = try!(File::open("/etc/dk-cli.conf").unwrap());

// 		let mut config_string = String::new();
// 	file.read_to_string(&mut config_string).unwrap();
// 	return serde_json::from_str(&config_string).unwrap().expect("config file is broken please rm /etc/dk-cli.conf");
// }