use std::process;

use clap::Parser;
use nslocer::Args;

fn main() {
    let args = Args::parse();

    match nslocer::run(args) {
        Ok(metrics) => match serde_json::to_string(&metrics) {
            Ok(json) => println!("{:#?}", json),
            Err(e) => {
                println!("Error parsing metrics");
                println!("{}", e);
                process::exit(1)
            }
        },
        Err(e) => {
            println!("Analyzer couldn't get metrics");
            println!("{}", e);
            process::exit(1)
        }
    }
}
