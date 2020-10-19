use std::process;
use minigrep::Config;
use minigrep::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cfg = minigrep::Config::new(&args).expect("Application error.");

    if let Err(e) = minigrep::run(cfg) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
