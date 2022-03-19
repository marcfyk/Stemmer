
pub fn is_vowel(letter: char) -> bool {
    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

pub fn is_consonant(letter: char) -> bool {
    match letter.to_ascii_lowercase() {
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' |
        'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' |
        's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! consonants { () => { String::from("bcdfghjklmnpqrstvwxyz") } }
    macro_rules! consonants_uppercase { () => { consonants!().to_uppercase() } }
    macro_rules! vowels { () => { String::from("aeiou") } }
    macro_rules! vowels_uppercase { () => { vowels!().to_uppercase() } }
    macro_rules! symbols { () => { String::from(" `~!@#$%^&*()_+/*-+\\|]}[{'\";:/?.>,<") } }

    #[test]
    fn is_consonant_input_consonant_lowercase() {
        for consonant in consonants!().chars() {
            assert!(is_consonant(consonant));
        }
    }

    #[test]
    fn is_consonant_input_consonant_uppercase() {
        for consonant in consonants_uppercase!().chars() {
            assert!(is_consonant(consonant));
        }
    }

    #[test]
    fn is_consonant_input_vowel_lowercase() {
        for vowel in vowels!().chars() {
            assert!(!is_consonant(vowel));
        }
    }

    #[test]
    fn is_consonant_input_vowel_uppercase() {
        for vowel in vowels_uppercase!().chars() {
            assert!(!is_consonant(vowel));
        }
    }

    #[test]
    fn is_consonant_input_symbol() {
        for symbol in symbols!().chars() {
            assert!(!is_consonant(symbol));
        }
    }

    #[test]
    fn is_vowel_input_vowel_lowercase() {
        for vowel in vowels!().chars() {
            assert!(is_vowel(vowel));
        }
    }

    #[test]
    fn is_vowel_input_vowel_uppercase() {
        for vowel in vowels_uppercase!().chars() {
            assert!(is_vowel(vowel));
        }
    }

    #[test]
    fn is_vowel_input_consonant_lowercase() {
        for consonant in consonants!().chars() {
            assert!(!is_vowel(consonant));
        }
    }

    #[test]
    fn is_vowel_input_consonant_uppercase() {
        for consonant in consonants_uppercase!().chars() {
            assert!(!is_vowel(consonant));
        }
    }

    #[test]
    fn is_vowel_input_symbol() {
        for symbol in symbols!().chars() {
            assert!(!is_vowel(symbol));
        }
    }

}

