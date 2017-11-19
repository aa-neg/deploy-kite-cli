
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_core::request::DispatchSignedRequest;
use rusoto_sqs::{Sqs,SqsClient, ListQueuesRequest, SendMessageRequest};

pub fn send_deployment() {
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
}