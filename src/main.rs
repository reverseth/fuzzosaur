use clap::Parser;
use reqwest;
use std::fs::File;
use std::io::{ErrorKind, Read};

/// Simple program to fuzz web servers… but in Rust !
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Wordlist to use
    #[clap(short, long)]
    wordlist: String,

    /// Target to fuzz
    #[clap(short, long)]
    target: String,
}

// Function which send the request to the web server and return a Result<reqwest::Response> object
async fn send_req(target_url: &String, word: &str) -> Result<reqwest::Response, reqwest::Error> {
    Ok(reqwest::get(target_url.to_string() + word).await?)
}

// Get a filename and return the words in a vector
fn file_to_vec(file_name: &str) -> Result<Vec<String>, ()> {
    // Creation of the file descriptor
    let mut file_descriptor: File = match File::open(&file_name) {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("The file \"{}\" can not be found", file_name);
            std::process::exit(1);
        }
        Err(ref error) if error.kind() == ErrorKind::PermissionDenied => {
            println!("You are not authorize to read the file : \"{}\"", file_name);
            std::process::exit(1);
        }
        Err(e) => {
            println!("Unknown error: {}", e);
            std::process::exit(1);
        }
    };

    // Read the file content and transform to a vector of lines
    let mut file_content: String = String::new();
    match file_descriptor.read_to_string(&mut file_content) {
        Err(_) => {
            println!("Could not read the file : \"{}\"", file_name);
            std::process::exit(1);
        }
        Ok(_) => (),
    }

    let file_vector: Vec<String> = file_content.split("\n").map(str::to_string).collect();

    Ok(file_vector)  // Result<Vec<String>, ()>
}

#[tokio::main]
async fn main() {

    // Parse CLI arguments
    let args: Args = Args::parse();

    // Get wordlist file and put words in a vector
    let fuzz_wordlist: Vec<String> = file_to_vec(args.wordlist.as_str()).unwrap();

    // Iterate on words to fuzz the target
    for word in fuzz_wordlist.iter() {
        let response: reqwest::Response = send_req(&args.target, &word).await.unwrap();

        // Check the HTTP Response code and act
        match response.status() {
            reqwest::StatusCode::OK => println!("{} : EXIST", word),
            reqwest::StatusCode::NOT_FOUND => (),
            _ => println!("{} : Unknown behavior", word),
        };
    }
}
