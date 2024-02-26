use clap::Parser;
use nslocer::Args;

fn main() {
    let args = Args::parse();

    match nslocer::run(args) {
        Ok(metrics) => println!("{:#?}", metrics),
        Err(_) => println!("Analyzer couldn't get metrics"),
    }
}
