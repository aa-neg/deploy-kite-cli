
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_sqs;

use hyper::header::{Headers, Connection, Authorization, Bearer};

use clap::{Arg, App};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use serde_json::{Value, Error};

use std::fs::File;
use std::io::Read;
use std::env;
use std::default::Default;

use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_core::request::DispatchSignedRequest;
use rusoto_sqs::{Sqs,SqsClient, ListQueuesRequest, SendMessageRequest};

fn main() {
	let build_kite_token :String = env::var("BUILD_KITE_TOKEN").expect("Missing build kite token.");
	env::var("AWS_ACCESS_KEY_ID").expect("missing aws env variables");
	env::var("AWS_SECRET_ACCESS_KEY").expect("missing aws env variables");

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

fn get_latest_build_number(build_kite_token: &str) {
	println!("Do we have a build kite token?: {}", build_kite_token);

	let mut latest_build_query  = format!(r#"{{
		"query": "query getLastetBuildNumber($slug_name: ID!) {{ pipeline (slug: $slug_name) {{ builds(first: 1,state: PASSED) {{ edges {{ node {{ number }} }} }} }} }}",
		"variables": "{{ \"slug_name\": \"siteminder/nexus2-admin-beef\" }}"
	}}"#);
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
}

fn send_sqs() {
	let provider = DefaultCredentialsProvider::new().unwrap();
	let client = SqsClient::new(default_tls_client().unwrap(), provider, Region::UsWest2);


	let mut message_body = format!(r#"
		{{
			"deploy":"{deploy}",
			"infrarepo":"{infrarepo}",
			"env":"{env}",
			"pipeline": "{pipeline}",
			"buildnumber":"{buildnumber}"
		}}
	"#,
		deploy=String::from("deploy"),
		infrarepo=String::from("testrepo"),
		env=String::from("testenv"),
		pipeline=String::from("testpipeline"),
		buildnumber=String::from("buildnumber")
	 );

	 println!("this is our message body: {}", message_body);

			// queue_url:String::from("https://sqs.us-west-2.amazonaws.com/145463046630/deploy-kite"),
	// let options = SendMessageRequest{
	// 		queue_url:String::from("https://sqs.us-west-2.amazonaws.com/145463046630/dk-cli-testing"),
	// 		message_body: message_body.to_owned(),
	// 		..Default::default()
	// 	};

	// match client.send_message(&options) {
	// 	Ok(output) => {
	// 		println!("Everything went find and we sent to the queue?");
	// 	},
	// 	Err(err) => {
	// 		println!("Could not get queues.");
	// 	}
	// }

	// let queue_options: ListQueuesRequest = Default::default();
	
	// match client.list_queues(&queue_options) {
	// 	Ok(output) => {
	// 		match output.queue_urls {
	// 			Some(queues) => {
	// 				for queue in queues {
	// 					println!("We have found some queues: {}", queue);
	// 				}
	// 			},
	// 			None => println!("We could not find any queues"),
	// 		}
	// 	},
	// 	Err(err) => {
	// 		println!("Could not get queues.");
	// 	}
	// };
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
