extern crate core;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate hyper_native_tls;

use std::env;
use std::thread;
use std::sync::Arc;

use hyper::client::Client;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;


#[derive(Serialize, Deserialize, Debug)]
struct GitHubUser {
    login: String,
    id: u64,
    url: String,
    name: String,
    email: String,
    followers: u64
}

fn load_user(client: Arc<hyper::Client>, username: String) -> Option<GitHubUser> {
    println!("Starting request for {}", username);
    let url = format!("{}{}", "https://api.github.com/users/", username);
    match client.get(&url).header(UserAgent(String::from("hyper-rust"))).send() {
        Ok(response) => {
            println!("End request for {}", username);
            serde_json::from_reader(response).ok()
        },
        Err(_) => None
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 {
        println!("Please enter a GitHub username");
        return;
    }

    let usernames = arguments[1..].iter().map(String::to_owned); 

    // HTTP client with ssl support
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Arc::new(Client::with_connector(connector));

    let thread_handles = usernames.map(|username| {
        let client_local = client.clone();
        thread::spawn(|| { load_user(client_local, username) })
    });

    for handle in thread_handles {
        let user = handle.join().unwrap();
        if let Some(user) = user {
            println!("{:?}", user);
            println!("");
        }
    }
}