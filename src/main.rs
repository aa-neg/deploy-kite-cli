
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

use operations::deploy;
use services::buildkite;
use services::buildkite::BuildDetails;
use services::awsSqs;

fn main() {
	// env::var("AWS_ACCESS_KEY_ID").expect("missing aws env variables");
	// env::var("AWS_SECRET_ACCESS_KEY").expect("missing aws env variables");
	env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable");

	// let client = reqwest::Client::new();

	// let details: BuildDetails = buildkite::get_latest_build_number(&client, String::from("siteminder/nexus2-admin-beef"));
	// println!("Our build number: {}", details.build_number);
	// println!("Our job id number: {}", details.job_uuid);
	// println!("Our job status number: {}", details.job_state);
	// awsSqs::send_deployment();

	// send_sqs();

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
		"list" => {
			operations::list::watched_collection();
		},
		"add" => {
			println!("trying to add");
			handle_add(matches.values_of("targets").unwrap());
		},
		"remove" => {
			println!("removing a config");
		},
		"deploy" => {
			deploy::target_pipeline(matches.values_of("targets").unwrap());
		}
		_  => println!("Operation not available.")
	}
}


fn get_config() {
	// println!("We currently live here: {:?}", std::env::current_exe());
	let mut file = File::open("config.json").unwrap();
	let mut config_string = String::new();
	file.read_to_string(&mut config_string).unwrap();
	
	let parsed: Value = serde_json::from_str(&config_string).unwrap();
	
	// println!("we parsed our config here is our pipelines: {}", parsed["pipelines"])
	
}

fn handle_add(pipelines: clap::Values) {
	println!("Requested to add the following");
	for pipeline in pipelines {
		println!("Your pipeline {}", pipeline);
	}
}
