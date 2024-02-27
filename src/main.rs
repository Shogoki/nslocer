use std::process;

use clap::Parser;
use nslocer::Args;

fn main() -> serde_json::Result<()> {
    let args = Args::parse();

    match nslocer::run(args) {
        Ok(metrics) => {
            let json = serde_json::to_string(&metrics)?;
            println!("{}", json);

            Ok(())
        }
        Err(e) => {
            println!("Analyzer couldn't get metrics");
            println!("{}", e);
            process::exit(1)
        }
    }
}
