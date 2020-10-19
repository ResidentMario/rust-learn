use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // args has to be a reference because Vector is a heap variable that doesn't implement the
        // Copy trait. Rust functions manage variables as stack objects, which must have a known size.
        // So [String] is not an acceptable function input type.
        //
        // So if we want the Config struct to exercise ownership over its contents, we have to clone
        // explicitly.
        //
        // Keeping a reference around is more efficient. But making a copy makes the borrow checker
        // less painful to deal with. In this case it is "worth it", according to the book.
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let cfg = Config { query: args[1].clone(), filename: args[2].clone() };
        Ok(cfg)
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut output: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            output.push(line);
        }
    }
    output
}

// this is a trait type declaration, we're writing a basic i/o script after 13-ish chapters of
// reading and we still don't know enough syntax to be able to do anything useful :'(
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // the ? operator bubbles any errors up to the caller (main() in this case).
    let contents = fs::read_to_string(config.filename)?;

    let result = search(&config.query, &contents);
    for line in result {
        println!("{}", line);
    }

    // println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
