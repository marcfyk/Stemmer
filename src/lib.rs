pub mod english;

pub trait Stem {
    fn stem(word: &str) -> String;
}

