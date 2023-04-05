use std::env;
use std::process;
use minigrep::run;
use minigrep::parse_config;
fn main() {
    let args:Vec<String>=env::args().collect();
    let config=parse_config(&args);
    println!("Searching for {}",config.query);
    println!("in file {}",config.path);
    if let Err(e)= run(config){
        eprintln!("Error in reading contents of the supplied file:{e}\n");
        process::exit(1);
    };
}

