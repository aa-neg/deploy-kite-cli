use serde_json::{Value, Error, from_str};
use hyper::header::{Headers, Connection, Authorization, Bearer};
use std::env;
use reqwest;
use std::io::Read;
use std::default::Default;

pub struct BuildDetails {
    pub build_number: String,
    pub job_uuid: String,
    pub job_state: String,
}

/*
To do match enum these states
*/
enum JobStates {
    PENDING,
    WAITING,
    WAITING_FAILED,
    BLOCKED,
    BLOCKED_FAILED,
    UNBLOCKED,
    UNBLOCKED_FAILED,
    LIMITED,
    SCHEDULED,
    ASSIGNED,
    ACCEPTED,
    RUNNING,
    FINISHED,
    CANCELING,
    CANCELED,
    TIMING_OUT,
    TIMED_OUT,
    SKIPPED,
    BROKEN,
}

pub fn get_latest_build_number(
    client: &reqwest::Client,
    pipeline: String,
    org: String,
) -> BuildDetails {

    let mut latest_build_query  = format!(r#"{{"query":
		"query getLastetBuildNumber($slug_name: ID!) {{ pipeline (slug: $slug_name) {{ builds(first: 1) {{ edges {{ node {{ number jobs(first: 2) {{ edges {{ node {{ ... on JobTypeBlock {{ uuid state }} }} }} }} }} }} }} }} }}",
		"variables": "{{ \"slug_name\": \"{org}/{pipeline}\" }}"
	}}"#,
        pipeline=pipeline,
        org=org
    );

    // println!("our latest build query: {}", latest_build_query);
    // let client = reqwest::Client::new();
    let mut res = client
        .post("https://graphql.buildkite.com/v1")
        .header(Authorization(Bearer {
            token: env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable"),
        }))
        .body(latest_build_query)
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let body: Value = from_str(&body).unwrap();

    println!("Fetched latest build details: {} ", body);

    let build_number = &body["data"]["pipeline"]["builds"]["edges"][0]["node"]["number"];

    let job_uuid = &body["data"]["pipeline"]["builds"]["edges"][0]["node"]["jobs"]["edges"][1]["node"]
        ["uuid"]
        .as_str()
        .unwrap();

    let job_state = &body["data"]["pipeline"]["builds"]["edges"][0]["node"]["jobs"]["edges"][1]["node"]
        ["state"]
        .as_str()
        .unwrap();

    println!("Found build number {}", build_number);
    println!("Found blocked job uuid {}", job_uuid);
    println!("Found job state {}", job_state);
    return BuildDetails {
        build_number: build_number.to_string(),
        job_uuid: job_uuid.to_lowercase(),
        job_state: job_state.to_uppercase(),
    };
}

pub fn unblock_build(client: &reqwest::Client, build_number: String, job_uuid: String, pipeline: String) {
    let mut query_url = format!(
        "https://api.buildkite.com/v2/organizations/siteminder/pipelines/{pipeline}/builds/{build_number}/jobs/{job_uuid}/unblock",
        build_number = build_number,
        job_uuid = job_uuid,
        pipeline = pipeline
    );

    println!("Our unblock build request: {}", query_url);


    let mut res = client
        .put(&query_url)
        .header(Authorization(Bearer {
            token: env::var("BUILD_KITE_TOKEN").expect("Missing build kite token env variable"),
        }))
        .body(String::from("{}"))
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let body: Value = from_str(&body).unwrap();

    println!("Finished our unblock request. {} ", body);
}
