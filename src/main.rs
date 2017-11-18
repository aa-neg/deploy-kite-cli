
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate reqwest;

use hyper::header::{Headers, Connection, Authorization, Bearer};

use clap::{Arg, App};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use serde_json::{Value, Error};

use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
	let build_kite_token :String = env::var("BUILD_KITE_TOKEN").expect("Missing build kite token.");
	println!("Do we have a build kite token?: {}", build_kite_token);

	let mut latest_build_query  = r#"{
		"query": "query getLastetBuildNumber($slug_name: ID!) { pipeline (slug: $slug_name) { builds(first: 1,state: PASSED) { edges { node { number } } } } }",
		"variables": "{ \"slug_name\": \"siteminder/nexus2-admin-beef\" }"
	}"# ;
		// Create a client.
	let client = reqwest::Client::new();
	let mut res = client.post("https://graphql.buildkite.com/v1")
			.header(Authorization(
				Bearer {
					token: build_kite_token.to_owned()
				}
			))
			.body(latest_build_query)
			.send().unwrap();

	// Read the Response.
	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	let body: Value = serde_json::from_str(&body).unwrap();

	println!("Finished our request.");
	let build_number = &body["data"]["pipeline"]["builds"]["edges"][0]["node"]["number"];

	println!("our build number {}", build_number);

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
			list_saved_items();
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
	let mut configString = String::new();
	file.read_to_string(&mut configString).unwrap();
	
	println!("Our json as a string!: {}", configString);
	
	let parsed: Value = serde_json::from_str(&configString).unwrap();
	
	println!("we parsed our config here is our pipelines: {}", parsed["pipelines"])
	
}

fn list_saved_items() {
	println!("You have requested some saved items");
	 // Create the table
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    // A more compicated way to add a row:
    table.add_row(Row::new(vec![
        Cell::new("foobar2"),
        Cell::new("bar2"),
        Cell::new("foo2")]));

    // Print the table to stdout
    table.printstd();
}

fn handle_add(pipelines: clap::Values) {
	println!("Requested to add the following");
	for pipeline in pipelines {
		println!("Your pipeline {}", pipeline);
	}
}
