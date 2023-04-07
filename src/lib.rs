use std::{fs, env};
use std::error::Error;
pub struct Config{
    pub query:String,
    pub path: String,
    pub ignore_case:bool,
}
impl Config{
    pub fn new(mut args:env::Args)->Result<Config,&'static str>{
        // if args.len()<3 {return Err("Not enough args");}
        args.next();
        let query=match args.next(){
            Some(q)=>q,
            None=>return Err("Didn't get a query string"),
        };
        let path=match args.next(){
            Some(p)=>p,
            None=>return Err("Didn't got a filename"),
        };
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(Config{
            query,
            path,
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

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    // let mut results=Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line.trim());
    //     }
    // }
    // results
    contents.lines().filter(|line|line.contains(query)).collect()
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