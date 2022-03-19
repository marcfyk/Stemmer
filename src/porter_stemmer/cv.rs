use super::utils::{
    is_consonant,
    is_vowel,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LetterType { C, V, Symbol }

impl From<char> for LetterType {

    fn from(c: char) -> Self {
        if is_consonant(c) {
            LetterType::C
        } else if is_vowel(c) {
            LetterType::V
        } else {
            LetterType::Symbol
        }
    }
}

pub fn cv_representation(letters: &[char]) -> Vec<LetterType> {
    let mut letter_types = letters.iter()
        .map(|letter| *letter)
        .map(|letter| LetterType::from(letter))
        .filter(|letter| *letter != LetterType::Symbol)
        .collect::<Vec<_>>();
    letter_types.dedup();
    letter_types
}

pub fn measure(cv_repr: &[LetterType]) -> usize {
    if cv_repr.is_empty() {
        return 0
    }
    let is_first_c = *cv_repr.first().unwrap() == LetterType::C;
    let is_back_v = *cv_repr.last().unwrap() == LetterType::V;
    if is_first_c && is_back_v {
        (cv_repr.len() - 2) / 2
    } else if is_first_c || is_back_v {
        (cv_repr.len() - 1) / 2
    } else {
        cv_repr.len() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cv_representation_single_vowel() {
        let input = String::from("a").chars().collect::<Vec<_>>();
        let expected = vec![LetterType::V];
        assert_eq!(expected, cv_representation(&input));
    }

    #[test]
    fn cv_representation_single_consonant() {
        let input = String::from("b").chars().collect::<Vec<_>>();
        let expected = vec![LetterType::C];
        assert_eq!(expected, cv_representation(&input));
    }

    #[test]
    fn cv_representation_consecutive_vowels() {
        let input = String::from("AeIoU").chars().collect::<Vec<_>>();
        let expected = vec![LetterType::V];
        assert_eq!(expected, cv_representation(&input))
    }

    #[test]
    fn cv_representation_consecutive_consonants() {
        let input = String::from("bCdFg").chars().collect::<Vec<_>>();
        let expected = vec![LetterType::C];
        assert_eq!(expected, cv_representation(&input))
    }

    #[test]
    fn cv_representation_word_with_consecutive_consonants_and_vowels() {
        let input = String::from("tree").chars().collect::<Vec<_>>();
        let expected = vec![
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(expected, cv_representation(&input));
    }

    #[test]
    fn cv_representation_word_with_multiple_consecutive_consonants_and_vowels() {
        let input = String::from("trouble").chars().collect::<Vec<_>>();
        let expected = vec![
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(expected, cv_representation(&input));
    }

    #[test]
    fn measure_empty_sequence() {
        let input = vec![];
        assert_eq!(0, measure(&input));
    }

    #[test]
    fn measure_single_c_sequence() {
        let input = vec![LetterType::C];
        assert_eq!(0, measure(&input));
    }

    #[test]
    fn measure_single_v_sequence() {
        let input = vec![LetterType::V];
        assert_eq!(0, measure(&input));
    }

    #[test]
    fn measure_start_with_c_end_with_v_zero_measure() {
        let input = vec![
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(0, measure(&input));
    }

    #[test]
    fn measure_start_with_c_end_with_v_single_measure() {
        let input = vec![
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(1, measure(&input));
    }

    #[test]
    fn measure_start_with_c_end_with_v_multiple_measure() {
        let input = vec![
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(2, measure(&input));
    }

    #[test]
    fn measure_start_with_v_end_with_c_single_measure() {
        let input = vec![
            LetterType::V,
            LetterType::C,
        ];
        assert_eq!(1, measure(&input));
    }

    #[test]
    fn measure_start_with_v_end_with_c_multiple_measure() {
        let input = vec![
            LetterType::V,
            LetterType::C,
            LetterType::V,
            LetterType::C,
        ];
        assert_eq!(2, measure(&input));
    }

    #[test]
    fn measure_start_with_v_end_with_v_single_measure() {
        let input = vec![
            LetterType::V,
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(1, measure(&input));
    }

    #[test]
    fn measure_start_with_v_end_with_v_multiple_measure() {
        let input = vec![
            LetterType::V,
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
        ];
        assert_eq!(2, measure(&input));
    }

    #[test]
    fn measure_start_with_c_end_with_c_single_measure() {
        let input = vec![
            LetterType::C,
            LetterType::V,
            LetterType::C,
        ];
        assert_eq!(1, measure(&input));
    }

    #[test]
    fn measure_start_with_c_end_with_c_multiple_measure() {
        let input = vec![
            LetterType::C,
            LetterType::V,
            LetterType::C,
            LetterType::V,
            LetterType::C,
        ];
        assert_eq!(2, measure(&input));
    }

}

