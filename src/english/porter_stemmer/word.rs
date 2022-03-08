use super::utils::{is_consonant, is_vowel};
use super::cv::{
    LetterType,
    cv_representation,
    measure,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Word {
    pub letters: Vec<char>,
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.letters.iter().collect()
    }
}

impl Word {
    pub fn new(word: &str) -> Self {
        let letters = word
            .chars()
            .filter(|c| is_consonant(*c) || is_vowel(*c))
            .collect::<Vec<char>>();
        Word { letters }
    }

    pub fn m(&self) -> usize {
        measure(&cv_representation(&self.letters))
    }

    pub fn ends_with(&self, substr: &str) -> bool {
        let letters_len = isize::try_from(self.letters.len()).unwrap_or_default();
        let substr_len = isize::try_from(substr.len()).unwrap_or_default();
        let diff = letters_len - substr_len;
        if diff < 0 {
            false
        } else {
            let diff = usize::try_from(diff);
            match diff {
                Ok(index) => {
                    self.letters[index..]
                        .iter()
                        .zip(substr.chars())
                        .all(|(x, y)| *x == y)
                },
                Err(_) => false,
            }
        }
    }


    pub fn ends_with_any(&self, values: &[&str]) -> bool {
        if self.letters.is_empty() {
            false
        } else {
            values.iter().any(|x| self.ends_with(x))
        }
    }

    pub fn ends_with_any_w_x_y(&self) -> bool {
        self.ends_with_any(&["w", "x", "y"])
    }

    pub fn ends_with_any_l_s_z(&self) -> bool {
        self.ends_with_any(&["l", "s", "z"])
    }

    pub fn contains_vowel(&self) -> bool {
        self.letters.iter().any(|x| is_vowel(*x))
    }

    pub fn has_double_consecutive_consonant_suffix(&self) -> bool {
        if self.letters.len() < 2 {
            false
        } else {
            let last_two = &self.letters[self.letters.len() - 2..];
            last_two.iter().all(|x| is_consonant(*x)) && last_two[0] == last_two[1]
        }
    }

    pub fn ends_with_types(&self, types: &[LetterType]) -> bool {
        let letters_len = isize::try_from(self.letters.len()).unwrap_or_default();
        let types_len = isize::try_from(types.len()).unwrap_or_default();
        let diff = letters_len - types_len;
        if diff < 0 {
            false
        } else {
            let diff = diff as usize;
            let diff = usize::try_from(diff);
            match diff {
                Ok(index) => {
                    self.letters[index..]
                        .iter()
                        .zip(types)
                        .all(|(letter, letter_type)| LetterType::from(*letter) == *letter_type)
                },
                Err(_) => false
            }
        }
    }

    pub fn ends_with_cvc(&self) -> bool {
        self.ends_with_types(&[LetterType::C, LetterType::V, LetterType::C])
    }

    pub fn replace_suffix(&self, n: usize, replacement: &str) -> Word {
        let end_index = std::cmp::max(self.letters.len() as isize - n as isize, 0);
        let truncated_word: String = self.letters[..end_index as usize]
            .iter()
            .collect();
        let padded_word = truncated_word + replacement;
        Word::new(&padded_word)
    }

    pub fn truncate_suffix(&self, n: usize) -> Word {
        self.replace_suffix(n, "")
    }

    pub fn pad_suffix(&self, pad: &str) -> Word {
        self.replace_suffix(0, pad)
    }

    pub fn index(&self, i: isize) -> Option<char> {
        let letters_len = isize::try_from(self.letters.len()).unwrap_or_default();
        let index = if i < 0 {
            match usize::try_from(letters_len - i.abs()) {
                Ok(index) => Some(index),
                Err(_) => None
            }
        } else if i < letters_len {
            Some(usize::try_from(i).unwrap_or_default())
        } else {
            None
        };
        match index {
            Some(n) => Some(self.letters[n]),
            None => None,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_word() {
        let word = "";
        let expected = Word { letters: vec![] };
        assert_eq!(expected, Word::new(word));
    }

    #[test]
    fn new_non_empty_word() {
        let word = "trees";
        let expected = Word { letters: vec!['t', 'r', 'e', 'e', 's'] };
        assert_eq!(expected, Word::new(word));
    }

    #[test]
    fn new_word_with_whitespace_and_tabspace() {
        let word = " t\t \t\t r ees \t ";
        let expected = Word { letters: vec!['t', 'r', 'e', 'e', 's'] };
        assert_eq!(expected, Word::new(word));
    }

    #[test]
    fn new_word_with_symbols() {
        let word = "*t|??r-ee's.";
        let expected = Word { letters: vec!['t', 'r', 'e', 'e', 's'] };
        assert_eq!(expected, Word::new(word));
    }

    #[test]
    fn to_string_empty_word() {
        assert_eq!("", Word::new("").to_string());
    }
    
    #[test]
    fn to_string_non_empty_word() {
        assert_eq!("trees", Word::new("trees").to_string());
    }

    #[test]
    fn ends_with_any_empty_word() {
        assert!(!Word::new("").ends_with_any(&vec!["a", "b", "c"]));
    }

    #[test]
    fn ends_with_any_word_no_match() {
        assert!(!Word::new("windows").ends_with_any(&vec!["a", "b", "c"]));
    }

    #[test]
    fn ends_with_any_word_match() {
        assert!(Word::new("mac").ends_with_any(&vec!["a", "b", "c"]));
    }

    #[test]
    fn contains_vowel_empty_word() {
        assert!(!Word::new("").contains_vowel());
    }

    #[test]
    fn contains_vowel_word_no_match() {
        assert!(!Word::new("ply").contains_vowel());
    }

    #[test]
    fn contains_vowel_word_match() {
        assert!(Word::new("pun").contains_vowel());
    }

    #[test]
    fn has_double_consecutive_consonant_suffix_empty_word() {
        assert!(!Word::new("").has_double_consecutive_consonant_suffix());
    }

    #[test]
    fn has_double_consecutive_consonant_suffix_word_no_match() {
        assert!(!Word::new("tree").has_double_consecutive_consonant_suffix());
    }

    #[test]
    fn has_double_non_consecutive_consonant_suffix_word_no_match() {
        assert!(!Word::new("own").has_double_consecutive_consonant_suffix());
    }

    #[test]
    fn has_double_consecutive_consonant_suffix_word_match() {
        assert!(Word::new("fizz").has_double_consecutive_consonant_suffix());
    }

    #[test]
    fn ends_with_types_empty_word() {
        assert!(!Word::new("").ends_with_types(&vec![LetterType::C, LetterType::V, LetterType::C]));
    }

    #[test]
    fn ends_with_types_word_no_match() {
        assert!(!Word::new("print").ends_with_types(&vec![LetterType::C, LetterType::V, LetterType::C]));
    }

    #[test]
    fn ends_with_types_word_match() {
        assert!(Word::new("shop").ends_with_types(&vec![LetterType::C, LetterType::V, LetterType::C]));
    }

    #[test]
    fn ends_with_empty_word() {
        assert!(!Word::new("").ends_with("es"));
    }

    #[test]
    fn ends_with_word_no_match() {
        assert!(!Word::new("ping").ends_with("es"));
    }

    #[test]
    fn ends_with_word_match() {
        assert!(Word::new("tries").ends_with("es"));
    }

    #[test]
    fn replace_suffix_larger_than_word() {
        assert_eq!(Word::new("print"), Word::new("abc").replace_suffix(5, "print"));
    }
    
    #[test]
    fn replace_suffix_full_replace_to_larger() {
        assert_eq!(Word::new("jumping"), Word::new("print").replace_suffix(5, "jumping"));
    }

    #[test]
    fn replace_suffix_full_replace_to_smaller() {
        assert_eq!(Word::new("print"), Word::new("jumping").replace_suffix(7, "print"));
    }

    #[test]
    fn replace_suffix_partial_replace_to_larger() {
        assert_eq!(Word::new("gaming"), Word::new("gamer").replace_suffix(2, "ing"));
    }

    #[test]
    fn replace_suffix_partial_replace_to_smaller() {
        assert_eq!(Word::new("gamer"), Word::new("gaming").replace_suffix(3, "er"));
    }

    #[test]
    fn truncate_suffix_larger_than_word() {
        assert_eq!(Word::new(""), Word::new("").truncate_suffix(1));
    }

    #[test]
    fn truncate_suffix_whole_length() {
        assert_eq!(Word::new(""), Word::new("game").truncate_suffix(4));
    }

    #[test]
    fn truncate_suffix_partial() {
        assert_eq!(Word::new("game"), Word::new("gamer").truncate_suffix(1));
    }

    #[test]
    fn truncate_suffix_zero_length() {
        assert_eq!(Word::new("game"), Word::new("game").truncate_suffix(0));
    }

    #[test]
    fn pad_suffix_on_empty_word() {
        assert_eq!(Word::new("print"), Word::new("").pad_suffix("print"));
    }

    #[test]
    fn pad_suffix_on_word() {
        assert_eq!(Word::new("printer"), Word::new("print").pad_suffix("er"));
    }

    #[test]
    fn ends_with_any_w_x_y_empty_no_match() {
        assert!(!Word::new("").ends_with_any_w_x_y());
    }

    #[test]
    fn ends_with_any_w_x_y_no_match() {
        assert!(!Word::new("truck").ends_with_any_w_x_y());
    }

    #[test]
    fn ends_with_any_w_x_y_match() {
        assert!(Word::new("willow").ends_with_any_w_x_y());
    }

    #[test]
    fn ends_with_any_l_s_z_empty_no_match() {
        assert!(!Word::new("").ends_with_any_l_s_z());
    }

    #[test]
    fn ends_with_any_l_s_z_no_match() {
        assert!(!Word::new("cry").ends_with_any_l_s_z());
    }

    #[test]
    fn ends_with_any_l_x_z_match() {
        assert!(Word::new("all").ends_with_any_l_s_z());
    }

    #[test]
    fn index_zero_empty_string() {
        assert_eq!(None, Word::new("").index(0));
    }

    #[test]
    fn index_negative_zero_empty_string() {
        assert_eq!(None, Word::new("").index(-0));
    }

    #[test]
    fn index_negative_empty_string() {
        assert_eq!(None, Word::new("").index(-1));
    }

    #[test]
    fn index_positive_empty_string() {
        assert_eq!(None, Word::new("").index(1));
    }

    #[test]
    fn index_positive_out_of_bounds() {
        assert_eq!(None, Word::new("run").index(5));
    }

    #[test]
    fn index_negative_out_of_bounds() {
        assert_eq!(None, Word::new("run").index(-5));
    }

    #[test]
    fn index_zero() {
        assert_eq!(Some('r'), Word::new("run").index(0));
    }

    #[test]
    fn index_negative_zero() {
        assert_eq!(Some('r'), Word::new("run").index(0));
    }

    #[test]
    fn index_positive() {
        assert_eq!(Some('i'), Word::new("running").index(4));
    }

    #[test]
    fn index_negative() {
        assert_eq!(Some('i'), Word::new("running").index(-3));
    }

}

