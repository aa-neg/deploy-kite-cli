
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;

use clap::{Arg, App};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use serde_json::{Value, Error};

use std::fs::File;
use std::io::Read;


fn main() {
	get_config();
	
	let matches = App::new("Deploy-kite cli")
		.version("0.1.0")
		.author("Arnold Agus")
		.about("deploy kite cli tool")
	.arg(Arg::with_name("operation")
		.index(1)
		.required(true)
		.possible_values(&["list", "add"])
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
			println!("too much code for now");
			handle_add(matches.values_of("add").unwrap());
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
