use crate::Stem;
use super::word::Word;
use super::rules;

pub struct PorterStemmer;

impl Stem for PorterStemmer {

    fn stem(word: &str) -> String {
        let w = Word::new(&word.to_lowercase());
        let steps: Vec<&dyn Fn(Word) -> Word> = vec![
            &rules::step_one_a,
            &rules::step_one_b,
            &rules::step_one_c,
            &rules::step_two,
            &rules::step_three,
            &rules::step_four,
            &rules::step_five_a,
            &rules::step_five_b,
        ];
        let result = steps.iter().fold(w, |w, f| f(w));
        result.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_match(input: &str, expected: &str) {
        assert_eq!(expected, PorterStemmer::stem(input));
    }

    #[test]
    fn stem_empty_word() {
        assert_match("", "");
    }

    #[test]
    fn stem_non_empty_word() {
        assert_match("reference", "refer");
    }

    #[test]
    fn stem_lowercase() {
        assert_match("traditional", "tradit");
    }

    #[test]
    fn stem_uppercase() {
        assert_match("MEETING", "meet");
    }

    #[test]
    fn stem_mixed_case() {
        assert_match("OwNeD", "own");
    }

    #[test]
    fn stem_with_whitespace_and_tabspace_ignore_whitespace() {
        assert_match(" i\ttem iz \t ation ", "item");
    }

    #[test]
    fn stem_with_symbols_ignore_symbols() {
        assert_match("*a!g?r|e-e_", "agre");
    }

}
