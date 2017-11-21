use clap;
use reqwest;
use services::buildkite;
use std::ascii::AsciiExt;

pub fn target_pipeline(pipelines: clap::Values) {
    
    let client = reqwest::Client::new();
    for pipeline in pipelines {
        let details: buildkite::BuildDetails = buildkite::get_latest_build_number(&client, pipeline.to_string(), String::from("siteminder"));
        buildkite::unblock_build(&client, details.build_number, details.job_uuid, pipeline.to_string());
    };
}