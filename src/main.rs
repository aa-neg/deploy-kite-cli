
extern crate clap;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_sqs;
#[macro_use]
extern crate prettytable;

use clap::{Arg, App};

use serde_json::{Value};

use std::fs::File;
use std::io::Read;
use std::env;

mod operations;
mod services;

use services::buildkite;
use services::awsSqs;

fn main() {
	env::var("AWS_ACCESS_KEY_ID").expect("missing aws env variables");
	env::var("AWS_SECRET_ACCESS_KEY").expect("missing aws env variables");
	env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable");

	buildkite::get_latest_build_number();
	awsSqs::send_deployment();

	// send_sqs();

	get_config();
	
	let matches = App::new("Deploy-kite cli")
		.version("0.1.0")
		.author("Arnold Agus")
		.about("deploy kite cli tool")
	.arg(Arg::with_name("operation")
		.index(1)
		.required(true)
		.possible_values(&["list", "add", "remove"])
		.help("an operation"))
	.arg(Arg::with_name("target")
		.short("a")
		.long("add")
		.index(2)
		.multiple(true)
		.takes_value(true)
		.help("target of previous command"))
	.get_matches();

	match matches.value_of("operation").unwrap() {
		"list" => {
			operations::list::watched_collection();
		},
		"add" => {
			handle_add(matches.values_of("add").unwrap());
		},
		"remove" => {
			println!("removing a config");

		},
		_  => println!("Operation not available.")
	}
}


fn get_config() {
	println!("We currently live here: {:?}", std::env::current_exe());
	let mut file = File::open("config.json").unwrap();
	let mut config_string = String::new();
	file.read_to_string(&mut config_string).unwrap();
	
	println!("Our json as a string!: {}", config_string);
	
	let parsed: Value = serde_json::from_str(&config_string).unwrap();
	
	println!("we parsed our config here is our pipelines: {}", parsed["pipelines"])
	
}

fn handle_add(pipelines: clap::Values) {
	println!("Requested to add the following");
	for pipeline in pipelines {
		println!("Your pipeline {}", pipeline);
	}
}
