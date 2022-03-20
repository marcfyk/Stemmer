mod utils;
pub mod porter_stemmer;

pub struct LanguageNotSupportedError;

impl std::fmt::Display for LanguageNotSupportedError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Language not supported")
    }
}

pub enum Language {
    English
}

pub trait Stem {
    fn stem(word: &str) -> String;
}

