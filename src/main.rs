use clap::Parser;

use serde::Serialize;
use tokei::{Config, LanguageType, Languages};
use transform::rust::transform_rust;

mod transform;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    language: Lang,
    #[allow(unused_parens)]
    #[arg(short, long,default_value_t = (".".to_string()) )]
    path: String,
    #[arg(short, long,default_value_t = ("tests".to_string())) ]
    excluded_path: String,
}
#[derive(Debug, Clone, clap::ValueEnum)]
enum Lang {
    Rust,
    Go,
    Solidity,
    Javascript,
}

#[derive(Debug, Clone, Serialize)]
struct CodeCount<'a> {
    file_name: &'a str,
    comment: usize,
    blank: usize,
    sloc: usize,
    nsloc: usize,
}

fn main() {
    let args = Args::parse();

    // The paths to search. Accepts absolute, relative, and glob paths.
    let paths = &[args.path];
    // Exclude any path that contains any of these strings.
    let excluded = &[&args.excluded_path[..]];
    // `Config` allows you to configure what is searched and counted.
    let mut config = Config::default();

    let mut languages = Languages::new();
    let result = match args.language {
        Lang::Rust => {
            languages.get_statistics(paths, excluded, &config);
            if !languages.contains_key(&LanguageType::Rust) {
                //early return empty vec if no Rust files are found
                vec![]
            } else {
                let sloc_reports = &languages[&LanguageType::Rust].reports.to_vec();

                languages = Languages::new();
                // passing in transform function
                config.transform_fn = Some(transform::transform);
                languages.get_statistics(paths, excluded, &config);

                languages[&LanguageType::Rust]
                    .reports
                    .iter()
                    .enumerate()
                    .map(|(i, rep)| CodeCount {
                        file_name: rep.name.to_str().expect("Can unpack path"),
                        comment: (*rep).stats.comments,
                        blank: (*rep).stats.blanks,
                        sloc: (sloc_reports[i]).stats.code,
                        nsloc: (*rep).stats.code,
                    })
                    .collect::<Vec<_>>()
            }
        }
        Lang::Javascript=> {
            languages.get_statistics(paths, excluded, &config);
            if !languages.contains_key(&LanguageType::JavaScript) {
                //early return empty vec if no matching files are found
                vec![]
            } else {
                let sloc_reports = &languages[&LanguageType::JavaScript].reports.to_vec();
                dbg!(&sloc_reports);

                languages = Languages::new();
                // passing in transform function
                config.transform_fn = Some(transform::transform);
                languages.get_statistics(paths, excluded, &config);

                languages[&LanguageType::JavaScript]
                    .reports
                    .iter()
                    .enumerate()
                    .map(|(i, rep)| CodeCount {
                        file_name: rep.name.to_str().expect("Can unpack path"),
                        comment: (*rep).stats.comments,
                        blank: (*rep).stats.blanks,
                        sloc: (sloc_reports[i]).stats.code,
                        nsloc: (*rep).stats.code,
                    })
                    .collect::<Vec<_>>()
            }
        }
        Lang::Solidity => {
            languages.get_statistics(paths, excluded, &config);
            if !languages.contains_key(&LanguageType::Solidity) {
                //early return empty vec if no matching files are found
                vec![]
            } else {
                let sloc_reports = &languages[&LanguageType::Solidity].reports.to_vec();
                dbg!(&sloc_reports);

                languages = Languages::new();
                // passing in transform function
                config.transform_fn = Some(transform::transform);
                languages.get_statistics(paths, excluded, &config);

                languages[&LanguageType::Solidity]
                    .reports
                    .iter()
                    .enumerate()
                    .map(|(i, rep)| CodeCount {
                        file_name: rep.name.to_str().expect("Can unpack path"),
                        comment: (*rep).stats.comments,
                        blank: (*rep).stats.blanks,
                        sloc: (sloc_reports[i]).stats.code,
                        nsloc: (*rep).stats.code,
                    })
                    .collect::<Vec<_>>()
            }
        }
        Lang::Go => {
            languages.get_statistics(paths, excluded, &config);
            if languages.contains_key(&LanguageType::Go) {
                languages[&LanguageType::Go]
                    .reports
                    .iter()
                    .map(|rep| CodeCount {
                        file_name: rep.name.to_str().expect("Can unpack path"),
                        comment: (*rep).stats.comments,
                        blank: (*rep).stats.blanks,
                        sloc: (*rep).stats.code,
                        nsloc: (*rep).stats.code,
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        }
    };

    let json_res = serde_json::to_string(&result).expect("Serialize Results");
    println!("{}", json_res);
}
