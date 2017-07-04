extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use clap::{Arg, App};

use std::thread;
use hyper::Client;

fn start_attack(){
    
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
            .help("Sets the target IPv4 address or DNS")
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
    println!("{:?}", threads);    
    
    let client = Client::new();
    let res = client.get("http://example.domain").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    
    
}