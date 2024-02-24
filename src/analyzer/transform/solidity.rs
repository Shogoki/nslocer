use regex::Regex;
use std::str;
use tokei::LanguageType;

pub fn transform_solidity(inp: &[u8], _: &LanguageType) -> String {
    //This regex is copied from solidity metrics: https://github.com/Consensys/solidity-metrics/blob/7b233b87c21abc29a0879a5dd6e8a3e29ac880f4/src/metrics/metrics.js#L640
    let regex = Regex::new(r"function\s*\S+\s*\([^{]*").expect("Failed to compile regex");
    let text = str::from_utf8(inp).expect("Failed to parse string");

    let a = regex.replace_all(text, "function ");
    a.to_string()
}
