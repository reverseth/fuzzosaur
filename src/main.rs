use std::fs::File;
use std::io::{ErrorKind, Read};
use reqwest;
use clap::Parser;

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

async fn send_req(target_url: &String, word: &str) -> Result<reqwest::Response, reqwest::Error> {
    Ok(reqwest::get(target_url.to_string() + word).await?)
}

fn file_to_vec(file_name: &str) -> Result<Vec<String>, ()> {

    // Création du descripteur de fichier
    let mut file_descriptor = match File::open(&file_name) {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("The file \"{}\" can not be found", file_name);
            std::process::exit(1);
        },
        Err(ref error) if error.kind() == ErrorKind::PermissionDenied => {
            println!("You are not authorize to read the file : \"{}\"", file_name);
            std::process::exit(1);
        },
        Err(e) => {
            println!("Unknown error: {}", e);
            std::process::exit(1);
        },
    };

    let mut file_content = String::new();
    match file_descriptor.read_to_string(&mut file_content) {
        Err(_) => {
            println!("Could not read the file : \"{}\"", file_name);
            std::process::exit(1);
        },
        Ok(_) => (),
    }
    
    let file_vector: Vec<String> = file_content.split("\n").map(str::to_string).collect();
    
    Ok(file_vector)
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    let fuzz_wordlist: Vec<String> = file_to_vec(args.wordlist.as_str()).unwrap();
    
    for word in fuzz_wordlist.iter() {
        let response = send_req(&args.target, &word).await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => println!("{} : EXIST", word),
            reqwest::StatusCode::NOT_FOUND => (),
            _ => println!("{} : Unknown behavior", word),
        };
    }
    
}
