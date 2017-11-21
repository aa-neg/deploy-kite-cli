
extern crate clap;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate prettytable;

use clap::{Arg, App};

use serde_json::{Value};

use std::fs::File;
use std::io::Read;
use std::env;

mod operations;
mod services;

use operations::deploy;
use services::buildkite;
use services::buildkite::BuildDetails;

fn main() {
	env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable");

	// get_config();
	
	let matches = App::new("Deploy-kite cli")
		.version("0.1.0")
		.author("Arnold Agus")
		.about("deploy kite cli tool")
	.arg(Arg::with_name("operation")
		.index(1)
		.required(true)
		.possible_values(&["list", "add", "remove", "deploy"])
		.help("an operation"))
	.arg(Arg::with_name("targets")
		.index(2)
		.multiple(true)
		.takes_value(true)
		.help("target of previous command"))
	.get_matches();

	match matches.value_of("operation").unwrap() {
		// "init" => {
		// 	match matches.occurences_of("targets") {
		// 		0 => operations::init::config();
		// 		_ => println!("Can't init with arguments")
		// 	}
		// },
		"deploy" => {
			deploy::target_pipeline(matches.values_of("targets").unwrap());
		}
		_  => println!("Operation not available.")
	}
}
