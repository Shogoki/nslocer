use serde::Serialize;
use tokei::{LanguageType, Languages};

mod transform;
use transform::{rust::transform_rust, solidity::transform_solidity};

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Lang {
    Solidity,
    Rust,
    Javascript,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileMetrics {
    file_name: String,
    comment: usize,
    blank: usize,
    nsloc: usize,
    lang: Lang,
}

pub struct Analyzer {
    lang: Lang,
}

impl Analyzer {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }

    fn transform(&self) -> fn(inp: &[u8], _: &LanguageType) -> String {
        match self.lang {
            Lang::Solidity => transform_solidity,
            Lang::Rust => transform_rust,
            Lang::Javascript => transform::transform,
        }
    }

    fn language_type(&self) -> LanguageType {
        match self.lang {
            Lang::Solidity => LanguageType::Solidity,
            Lang::Rust => LanguageType::Rust,
            Lang::Javascript => LanguageType::JavaScript,
        }
    }

    pub fn analyze(&self, paths: &Vec<String>, excluded: &Vec<String>) -> Vec<FileMetrics> {
        let mut languages = Languages::new();
        let mut config = tokei::Config::default();

        // Transform code into a specific format before analyzing
        let transform = || self.transform();
        config.transform_fn = Some(transform());

        languages.get_statistics(
            &paths,
            &excluded.iter().map(|p| p.as_str()).collect::<Vec<&str>>(),
            &config,
        );

        match languages.get(&self.language_type()) {
            Some(language) => language
                .reports
                .iter()
                .enumerate()
                .map(|(_, rep)| FileMetrics {
                    file_name: rep.name.to_str().expect("Couldn't unpack path").to_string(),
                    comment: (*rep).stats.comments,
                    blank: (*rep).stats.blanks,
                    nsloc: (*rep).stats.code,
                    lang: self.lang,
                })
                .collect::<Vec<FileMetrics>>(),

            None => vec![],
        }
    }

    /*
     * TODO:
     * - Render reports
     * - Analyze imports
     */
}
