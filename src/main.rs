extern crate docopt;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;

use docopt::Docopt;
use futures::{	StreamExt, 
				prelude::{Future, stream}
			};
use reqwest::{Client, Error,get, Response};
use serde::Deserialize;
use serde_json::{Value};


const POST_URI: &'static str = "http://localhost:5000";
const RANDOM_NAME_URI: &'static str = "https://names.mcquay.me/api/v0";


const USAGE: &'static str = "
Usage:
	joke_collector [options]

Options:
	--num-requests=<n>   Number of concurrent requests [default: 1].
	-h, --help           Show this screen.
	--version            Show version.

";

#[derive(Debug, Deserialize)]
struct Args {
	flag_num_requests: usize,
}

#[tokio::main]
async fn main() {

	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.deserialize())
		.unwrap_or_else(|e| e.exit());

	let num_requests = args.flag_num_requests;

	// TODO: Setup and bind server to the provided post uri (including routes).      
	    
    // Get a name and the corresponding joke
	let name_uris = vec![RANDOM_NAME_URI; num_requests];

	let fut = stream::iter(name_uris).for_each_concurrent(
		num_requests,
		|name_uri| async move {

   			let name_body = get_body(name_uri.to_string()).await;
			let name_json: Value = serde_json::from_str(&name_body).expect("Unable to parse to json.");
			let (first_name, last_name) = (&name_json["first_name"], &name_json["last_name"]);
			
			let joke_uri = make_joke_uri(first_name, last_name);
			let joke_body = get_body(joke_uri.to_string()).await;
			let joke_json: Value = serde_json::from_str(&joke_body).expect("Unable to parse to json.");
			let joke = joke_json["value"]["joke"].as_str().unwrap_or("");
			
			println!("Joke to post: {:?}", joke.to_string());

			// TODO: Use the following to post joke to uri.
			let client = Client::new();
			let _joke_post_body = post_body(client, joke.to_string()).await;
		}
	);

	fut.await;

}

pub fn get_body(uri: String) -> impl Future<Output = String>  {
	async move {
		let body = get(&uri)
		    .await
		    .expect("get")
		    .text()
		    .await
		    .expect("text");
		body
	}
}

pub fn post_body(client: Client, body: String) -> impl Future<Output = Result<Response, Error>>  {
	async move {
		let res = client.post(POST_URI)
			.body(body)
		    .send()
		    .await;
		res
	}
}

fn make_joke_uri(first_name: &Value, last_name: &Value) -> String {
	let first_name = first_name.as_str().unwrap_or("");
	let last_name = last_name.as_str().unwrap_or("");
	"http://api.icndb.com/jokes/random?firstName=".to_string() + first_name + "&lastName=" + last_name + "&limitTo=[nerdy]"
}

