use std::{
    io::{ErrorKind, Write},
    process::{Command, Stdio},
};

use tokei::LanguageType;

pub fn transform_rust(_: &[u8], _: &LanguageType) -> String {
    // this is only for rust

    let mut fmt_cmd = Command::new("rustfmt");
    fmt_cmd.stdin(Stdio::piped());
    fmt_cmd.stdout(Stdio::piped());
    fmt_cmd.arg("--backup");
    fmt_cmd.arg("--edition");
    fmt_cmd.arg("2021");
    fmt_cmd.arg("--config");
    fmt_cmd.arg("fn_single_line=true,imports_granularity=Crate,max_width=999999999,use_small_heuristics=Max,where_single_line=true");
    /* rustfmt config we want to use
    fn_single_line=true
    imports_granularity="Crate"
    max_width=999999999 # This sets the max width a line can have before rustfmt breaks it.
    use_small_heuristics="Max" # This makes all line widths equal max_width
    where_single_line=true
    */
    let child = fmt_cmd.spawn().expect("Failed to spawn rustfmt");

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                panic!("rustfmt was not found! Please install or check PATH!");
            } else {
                panic!("Unkown error occured spawning rustfmt!");
            }
        }
    };

    let res = String::from_utf8(output.stdout).expect("Should be valid String");
    res
}
