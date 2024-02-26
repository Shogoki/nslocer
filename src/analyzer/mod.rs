use serde::Serialize;
use tokei::{LanguageType, Languages};

mod transform;
use transform::{rust::transform_rust, solidity::transform_solidity};

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Lang {
    Solidity,
    Rust,
    Javascript,
    Go,
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

    fn transform(&self) -> Option<fn(inp: &[u8], _: &LanguageType) -> String> {
        match self.lang {
            Lang::Solidity => Some(transform_solidity),
            Lang::Rust => Some(transform_rust),
            Lang::Javascript => Some(transform_solidity),
            Lang::Go => None,
        }
    }

    fn language_type(&self) -> LanguageType {
        match self.lang {
            Lang::Solidity => LanguageType::Solidity,
            Lang::Rust => LanguageType::Rust,
            Lang::Javascript => LanguageType::JavaScript,
            Lang::Go => LanguageType::Go,
        }
    }

    pub fn analyze(&self, paths: &Vec<String>, excluded: &Vec<String>) -> Vec<FileMetrics> {
        let mut languages = Languages::new();
        let mut config = tokei::Config::default();

        // Transform code into a specific format before analyzing
        config.transform_fn = self.transform();
        config.types = Some(vec![self.language_type()]);

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
     * - Comments ratio
     * - Render reports
     * - Analyze imports
     */
}
