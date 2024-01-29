use std::{io::ErrorKind, path::PathBuf, process::Command};

use clap::Parser;

use serde::Serialize;
use tokei::{Config, LanguageType, Languages};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    language: Lang,
    #[arg(short, long,default_value_t = (".".to_string()) )]
    path: String,
    #[arg(short, long,default_value_t = ("tests".to_string())) ]
    excluded_path: String,
}
#[derive(Debug, Clone, clap::ValueEnum)]
enum Lang {
    Rust,
    Go,
}

#[derive(Debug, Clone, Serialize)]
struct CodeCount<'a> {
    file_name: &'a str,
    comment: usize,
    blank: usize,
    sloc: usize,
    nsloc: usize,
}

fn restore_rust_file(path: &PathBuf) {
    // we check if there is a .bk file that was generated by rustfmt, if yes we will restore it
    let mut backup = path.to_owned();
    backup.set_extension("bk");
    if backup.exists() {
        //This will actually overwrite the file
        let _ = std::fs::rename(backup, path);
    }
}

fn main() {
    let args = Args::parse();

    // The paths to search. Accepts absolute, relative, and glob paths.
    let paths = &[args.path];
    // Exclude any path that contains any of these strings.
    let excluded = &[&args.excluded_path[..]];
    // `Config` allows you to configure what is searched and counted.
    let config = Config::default();

    let mut languages = Languages::new();
    let result = match args.language {
        Lang::Rust => {
            languages.get_statistics(paths, excluded, &config);
            if !languages.contains_key(&LanguageType::Rust) {
                //early return empty vec if no Rust files are found
                vec![]
            } else {
                let sloc_reports = &languages[&LanguageType::Rust].reports.to_vec();

                let files = sloc_reports
                    .iter()
                    .map(|r| r.name.to_str().expect("Failed to unwrap Rustfile path"));

                let mut fmt_cmd = Command::new("rustfmt");
                fmt_cmd.arg("--backup");
                fmt_cmd.arg("--edition");
                fmt_cmd.arg("2021");
                fmt_cmd.arg("--config");
                fmt_cmd.arg("fn_single_line=true,imports_granularity=Crate,max_width=999999999,use_small_heuristics=Max,where_single_line=true");
                fmt_cmd.args(files);
                /* rustfmt config we want to use
                fn_single_line=true
                imports_granularity="Crate"
                max_width=999999999 # This sets the max width a line can have before rustfmt breaks it.
                use_small_heuristics="Max" # This makes all line widths equal max_width
                where_single_line=true
                */
                let _output = match fmt_cmd.output() {
                    Ok(o) => o, //TODO: Check for exit codes
                    Err(e) => {
                        if e.kind() == ErrorKind::NotFound {
                            panic!("rustfmt was not found! Please install or check PATH!");
                        } else {
                            panic!("Unkown error occured spawning rustfmt!");
                        }
                    }
                };

                languages = Languages::new();
                languages.get_statistics(paths, excluded, &config);

                languages[&LanguageType::Rust]
                    .reports
                    .iter()
                    .enumerate()
                    .map(|(i, rep)| {
                        //restoring original rust file
                        restore_rust_file(&rep.name);
                        CodeCount {
                        file_name: rep.name.to_str().expect("Can unpack path"),
                        comment: (*rep).stats.comments,
                        blank: (*rep).stats.blanks,
                        sloc: (sloc_reports[i]).stats.code,
                        nsloc: (*rep).stats.code,
                    }})
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
