use tokei::LanguageType;

use self::{rust::transform_rust, solidity::transform_solidity};

pub mod rust;
pub mod solidity;

// Generic Transform function
pub fn transform(inp: &[u8], lang_type: &LanguageType) -> String {
    match lang_type {
        &LanguageType::Rust => transform_rust(inp, lang_type),
        &LanguageType::Solidity => transform_solidity(inp, lang_type),
        // we can use the same normalization for js like for solidity
        &LanguageType::JavaScript => transform_solidity(inp, lang_type),
        _ => String::from_utf8_lossy(inp).to_string(),
    }
}
