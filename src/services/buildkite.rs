use serde_json::{Value, Error, from_str};
use hyper::header::{Headers, Connection, Authorization, Bearer};
use std::env;
use reqwest;
use std::io::Read;
use std::default::Default;

pub fn get_latest_build_number() {

	let mut latest_build_query  = format!(r#"{{
		"query": "query getLastetBuildNumber($slug_name: ID!) {{ pipeline (slug: $slug_name) {{ builds(first: 1,state: PASSED) {{ edges {{ node {{ number }} }} }} }} }}",
		"variables": "{{ \"slug_name\": \"siteminder/nexus2-admin-beef\" }}"
	}}"#);
	let client = reqwest::Client::new();
	let mut res = client.post("https://graphql.buildkite.com/v1")
		.header(Authorization(
			Bearer {
				token: env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable")
			}
		))
		.body(latest_build_query)
		.send().unwrap();

	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	let body: Value = from_str(&body).unwrap();

	println!("Finished our request.");
	let build_number = &body["data"]["pipeline"]["builds"]["edges"][0]["node"]["number"];

	println!("our build number {}", build_number);
}