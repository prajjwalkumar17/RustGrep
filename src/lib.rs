use std::fs;
use std::process;
use std::error::Error;
use std::env;
pub struct Config<'a>{
    pub query:&'a str,
    pub path:&'a str,
    pub ignore_case:bool,
}
impl<'a> Config<'a>{
    pub fn new(args:&'a [String])->Result<Config<'a>,&'static str>{
        if args.len()<3 {return Err("Not enough args");}
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(Config{
            query:&args[1],
            path:&args[2],
            ignore_case,
        })
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents =fs::read_to_string(config.path)?;
    // println!("\nWith text as\n{}",contents);
    let results=if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };
 
    for line in results{
        println!("{line}");
    }
    Ok(())
}
pub fn parse_config<'a>(args:&'a [String])->Config<'a>{
    Config::new(args).unwrap_or_else(|err|{
        eprintln!("Some error in args: {err}");
        process::exit(1);
    })
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line.trim());

        }
    }
    results
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    let mut results=Vec::new();
    let query =query.to_ascii_lowercase();
    for lines in contents.lines(){
        if lines.to_ascii_lowercase().contains(&query){
            results.push(lines.trim());
        }

    }
    results
}

//TODO Tests
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive(){
        let query="duct";
        let contents="\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
    #[test]
    fn case_insensitive(){
        let query="rUSt";
        let contents="\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me..";
        assert_eq!(vec!["Rust:","Trust me.."],search_case_insensitive(query,contents));
    }
}