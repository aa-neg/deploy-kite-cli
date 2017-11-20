use clap;
use reqwest;
use services::buildkite;
use std::ascii::AsciiExt;

pub fn target_pipeline(pipelines: clap::Values) {
    println!("You have requested some saved items");
    
    let client = reqwest::Client::new();
    for pipeline in pipelines {
        let details: buildkite::BuildDetails = buildkite::get_latest_build_number(&client, pipeline.to_string(), String::from("siteminder"));


        buildkite::unblock_build(&client, details.build_number, details.job_uuid);
   
        // match details {
        //     buildkite::BuildDetails { job_state : "BLOCKED", .. } => {
        //         buildkite::unblock_build(&client, details.build_number, details.job_uuid);
        //     },
        //     _ => {
        //         println!("Invalid job state {}", details.job_state);
        //     },
        // };
    };
}