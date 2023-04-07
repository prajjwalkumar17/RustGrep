use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;
fn main() {
    let config=Config::new(env::args()).unwrap_or_else(|_err|{
        println!("Problem parsing args");
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("in file {}",config.path);
    if let Err(e)= run(config){
        eprintln!("Error in reading contents of the supplied file:{e}\n");
        process::exit(1);
    };
}

