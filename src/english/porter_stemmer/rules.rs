use super::word::Word;

pub fn step_one_a(word: Word) -> Word {
    if word.ends_with("sses") {
        word.truncate_suffix(2)
    } else if word.ends_with("ies") {
        word.truncate_suffix(2)
    } else if word.ends_with("ss") {
        word
    } else if word.ends_with("s") {
        word.truncate_suffix(1)
    } else {
        word
    }
}

fn step_one_b_update(word: Word) -> Word {
    if word.ends_with("at") {
        word.pad_suffix("e")
    } else if word.ends_with("bl") {
        word.pad_suffix("e")
    } else if word.ends_with("iz") {
        word.pad_suffix("e")
    } else {
        let truncated_to_single_letter = word.truncate_suffix(1);
        if word.has_double_consecutive_consonant_suffix() && !word.ends_with_any_l_s_z() {
            truncated_to_single_letter
        } else {
            let pad_with_e = word.pad_suffix("e");
            if word.m() == 1 && word.ends_with_cvc() && !word.ends_with_any_w_x_y() {
                pad_with_e
            } else {
                word
            }
        }
    }
}

pub fn step_one_b(word: Word) -> Word {
    if word.ends_with("eed") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 0 {
            prefix.pad_suffix("ee")
        } else {
            word
        }
    } else if word.ends_with("ed") {
        let prefix = word.truncate_suffix(2);
        if prefix.contains_vowel() {
            step_one_b_update(prefix)
        } else {
            word
        }
    } else if word.ends_with("ing") {
        let prefix = word.truncate_suffix(3);
        if prefix.contains_vowel() {
            step_one_b_update(prefix)
        } else {
            word
        }
    } else {
        word
    }
}

pub fn step_one_c(word: Word) -> Word {
    if word.ends_with("y") {
        let prefix = word.truncate_suffix(1);
        if prefix.contains_vowel() {
            prefix.pad_suffix("i")
        } else {
            word
        }
    } else {
        word
    }
}

fn step_two_a(word: Word) -> Word {
    if word.ends_with("ational") {
        let prefix = word.truncate_suffix(7);
        if prefix.m() > 0 { prefix.pad_suffix("ate") } else { word }
    } else if word.ends_with("tional") {
        let prefix = word.truncate_suffix(6);
        if prefix.m() > 0 { word.truncate_suffix(2) } else { word }
    } else {
        word
    }
}

fn step_two_c(word: Word) -> Word {
    if word.ends_with("enci") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { word.replace_suffix(1, "e") } else { word }
    } else if word.ends_with("anci") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { word.replace_suffix(1, "e") } else { word }
    } else {
        word
    }
}

fn step_two_e(word: Word) -> Word {
    if word.ends_with("izer") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { word.truncate_suffix(1) } else { word }
    } else {
        word
    }
}

fn step_two_l(word: Word) -> Word {
    if word.ends_with("abli") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { word.replace_suffix(1, "e") } else { word }
    } else if word.ends_with("alli") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { word.truncate_suffix(2) } else { word }
    } else if word.ends_with("entli") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { word.truncate_suffix(2) } else { word }
    } else if word.ends_with("eli") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 0 { prefix.pad_suffix("e") } else { word }
    } else if word.ends_with("ousli") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { word.truncate_suffix(2) } else { word }
    } else {
        word
    }
}

fn step_two_o(word: Word) -> Word {
    if word.ends_with("ization") {
        let prefix = word.truncate_suffix(7);
        if prefix.m() > 0 { prefix.pad_suffix("ize") } else { word }
    } else if word.ends_with("ation") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("ate") } else { word }
    } else if word.ends_with("ator") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { prefix.pad_suffix("ate") } else { word }
    } else {
        word
    }
}

fn step_two_s(word: Word) -> Word {
    if word.ends_with("alism") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("al") } else { word }
    } else if word.ends_with("iveness") {
        let prefix = word.truncate_suffix(7);
        if prefix.m() > 0 { prefix.pad_suffix("ive") } else { word }
    } else if word.ends_with("fulness") {
        let prefix = word.truncate_suffix(7);
        if prefix.m() > 0 { prefix.pad_suffix("ful") } else { word }
    } else if word.ends_with("ousness") {
        let prefix = word.truncate_suffix(7);
        if prefix.m() > 0 { prefix.pad_suffix("ous") } else { word }
    } else {
        word
    }
}

fn step_two_t(word: Word) -> Word {
    if word.ends_with("aliti") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("al") } else { word }
    } else if word.ends_with("iviti") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("ive") } else { word }
    } else if word.ends_with("biliti") {
        let prefix = word.truncate_suffix(6);
        if prefix.m() > 0 { prefix.pad_suffix("ble") } else { word }
    } else {
        word
    }
}

pub fn step_two(word: Word) -> Word {
    let penultimate_char = word.index(-2);
    if let None = penultimate_char {
        return word
    }
    let penultimate_char = penultimate_char.unwrap();
    match penultimate_char {
        'a' => step_two_a(word),
        'c' => step_two_c(word),
        'e' => step_two_e(word),
        'l' => step_two_l(word),
        'o' => step_two_o(word),
        's' => step_two_s(word),
        't' => step_two_t(word),
        _ => word,
    }
}

pub fn step_three(word: Word) -> Word {
    if word.ends_with("icate") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("ic") } else { word }
    } else if word.ends_with("ative") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix } else { word }
    } else if word.ends_with("alize") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("al") } else { word }
    } else if word.ends_with("iciti") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 0 { prefix.pad_suffix("ic") } else { word }
    } else if word.ends_with("ical") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { prefix.pad_suffix("ic") } else { word }
    } else if word.ends_with("ful") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 0 { prefix } else { word }
    } else if word.ends_with("ness") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 0 { prefix } else { word }
    } else {
        word
    }
}


fn step_four_a(word: Word) -> Word {
    if word.ends_with("al") {
        let prefix = word.truncate_suffix(2);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_c(word: Word) -> Word {
    if word.ends_with("ance") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("ence") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_e(word: Word) -> Word {
    if word.ends_with("er") {
        let prefix = word.truncate_suffix(2);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_i(word: Word) -> Word {
    if word.ends_with("ic") {
        let prefix = word.truncate_suffix(2);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_l(word: Word) -> Word {
    if word.ends_with("able") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("ible") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_n(word: Word) -> Word {
    if word.ends_with("ant") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("ement") {
        let prefix = word.truncate_suffix(5);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("ment") {
        let prefix = word.truncate_suffix(4);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("ent") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_o(word: Word) -> Word {
    if word.ends_with("ion") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 && prefix.ends_with_any(&["s", "t"]) { prefix } else { word }
    } else if word.ends_with("ou") {
        let prefix = word.truncate_suffix(2);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_s(word: Word) -> Word {
    if word.ends_with("ism") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_t(word: Word) -> Word {
    if word.ends_with("ate") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else if word.ends_with("iti") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_u(word: Word) -> Word {
    if word.ends_with("ous") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_v(word: Word) -> Word {
    if word.ends_with("ive") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

fn step_four_z(word: Word) -> Word {
    if word.ends_with("ize") {
        let prefix = word.truncate_suffix(3);
        if prefix.m() > 1 { prefix } else { word }
    } else {
        word
    }
}

pub fn step_four(word: Word) -> Word {
    let penultimate_char = word.index(-2);
    if let None = penultimate_char {
        return word
    }
    let penultimate_char = penultimate_char.unwrap();
    match penultimate_char {
        'a' => step_four_a(word),
        'c' => step_four_c(word),
        'e' => step_four_e(word),
        'i' => step_four_i(word),
        'l' => step_four_l(word),
        'n' => step_four_n(word),
        'o' => step_four_o(word),
        's' => step_four_s(word),
        't' => step_four_t(word),
        'u' => step_four_u(word),
        'v' => step_four_v(word),
        'z' => step_four_z(word),
        _ => word,
    }
}

pub fn step_five_a(word: Word) -> Word {
    if word.ends_with("e") {
        let prefix = word.truncate_suffix(1);
        if prefix.m() > 1 { 
            prefix 
        } else if prefix.m() == 1 && !prefix.ends_with_cvc() || prefix.ends_with_any_w_x_y() { 
            prefix
        } else {
            word
        }
    } else {
        word
    }
}

pub fn step_five_b(word: Word) -> Word {
    if word.m() > 1 && word.has_double_consecutive_consonant_suffix() && word.ends_with("l") {
        word.truncate_suffix(1)
    } else {
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_match(f: fn(Word) -> Word, input_word: &str, expected_word: &str) {
        assert_eq!(Word::new(expected_word), f(Word::new(input_word)));
    }

    #[test]
    fn step_one_a_sses() {
        assert_match(step_one_a, "presses", "press");
    }

    #[test]
    fn step_one_a_ies() {
        assert_match(step_one_a, "ties", "ti");
    }

    #[test]
    fn step_one_a_ss() {
        assert_match(step_one_a, "press", "press");
    }

    #[test]
    fn step_one_a_s() {
        assert_match(step_one_a, "dogs", "dog");
    }

    #[test]
    fn step_one_a_no_match() {
        assert_match(step_one_a, "cartographer", "cartographer");
    }

    #[test]
    fn step_one_b_eed_m_zero() {
        assert_match(step_one_b, "feed", "feed");
    }

    #[test]
    fn step_one_b_eed_m_more_than_zero() {
        assert_match(step_one_b, "agreed", "agree");
    }

    #[test]
    fn step_one_b_ed_contains_vowel() {
        assert_match(step_one_b, "plastered", "plaster");
    }

    #[test]
    fn step_one_b_ed_no_vowel() {
        assert_match(step_one_b, "bled", "bled");
    }

    #[test]
    fn step_one_b_ed_contains_vowel_with_update_at() {
        assert_match(step_one_b, "conflated", "conflate");
    }

    #[test]
    fn step_one_b_ed_contains_vowel_with_update_bl() {
        assert_match(step_one_b, "troubled", "trouble");
    }

    #[test]
    fn step_one_b_ed_contains_vowel_with_update_iz() {
        assert_match(step_one_b, "sized", "size");
    }

    #[test]
    fn step_one_b_ed_contains_vowel_with_update_double_consecutive_consonant_does_not_end_with_lsz() {
        assert_match(step_one_b, "tanned", "tan");
    }

    #[test]
    fn step_one_b_ed_contains_vowel_with_update_double_consecutive_consonant_ends_with_lsz() {
        assert_match(step_one_b, "fizzed", "fizz");
    }

    #[test]
    fn step_one_b_ing_contains_vowel() {
        assert_match(step_one_b, "motoring", "motor");
    }

    #[test]
    fn step_one_b_ing_no_vowel() {
        assert_match(step_one_b, "sing", "sing");
    }

    #[test]
    fn step_one_b_ing_contains_vowel_with_update_double_consecutive_consonant_does_not_end_with_lsz() {
        assert_match(step_one_b, "hopping", "hop");
    }

    #[test]
    fn step_one_b_ing_contains_vowel_with_no_update_double_non_consecutive_consonant_does_not_end_with_lsz() {
        assert_match(step_one_b, "owning", "own");
    }

    #[test]
    fn step_one_b_ing_contains_vowel_with_update_double_consecutive_consonant_ends_with_lsz() {
        assert_match(step_one_b, "hissing", "hiss");
    }

    #[test]
    fn step_one_b_ing_contains_vowel_with_update_e() {
        assert_match(step_one_b, "filing", "file");
    }

    #[test]
    fn step_one_b_ing_contains_vowel_no_update_e() {
        assert_match(step_one_b, "failing", "fail");
    }

    #[test]
    fn step_one_c_contains_vowel() {
        assert_match(step_one_c, "happy", "happi");
    }

    #[test]
    fn step_one_c_no_vowel() {
        assert_match(step_one_c, "sky", "sky");
    }

    #[test]
    fn step_two_ational() {
        assert_match(step_two, "relational", "relate");
    }

    #[test]
    fn step_two_tional() {
        assert_match(step_two, "conditional", "condition");
    }

    #[test]
    fn step_two_tional_no_update() {
        assert_match(step_two, "rational", "rational");
    }

    #[test]
    fn step_two_enci() {
        assert_match(step_two, "valenci", "valence");
    }

    #[test]
    fn step_two_anci() {
        assert_match(step_two, "hesitanci", "hesitance");
    }

    #[test]
    fn step_two_izer() {
        assert_match(step_two, "digitizer", "digitize");
    }

    #[test]
    fn step_two_abli() {
        assert_match(step_two, "conformabli", "conformable");
    }

    #[test]
    fn step_two_alli() {
        assert_match(step_two, "radicalli", "radical");
    }

    #[test]
    fn step_two_entli() {
        assert_match(step_two, "differentli", "different");
    }

    #[test]
    fn step_two_eli() {
        assert_match(step_two, "vileli", "vile");
    }

    #[test]
    fn step_two_ousli() {
        assert_match(step_two, "analogousli", "analogous");
    }

    #[test]
    fn step_two_izlation() {
        assert_match(step_two, "vietnamization", "vietnamize");
    }

    #[test]
    fn step_two_ation() {
        assert_match(step_two, "predication", "predicate");
    }

    #[test]
    fn step_two_ator() {
        assert_match(step_two, "operator", "operate");
    }

    #[test]
    fn step_two_alism() {
        assert_match(step_two, "feudalism", "feudal");
    }

    #[test]
    fn step_two_iveness() {
        assert_match(step_two, "decisiveness", "decisive");
    }

    #[test]
    fn step_two_fulness() {
        assert_match(step_two, "hopefulness", "hopeful");
    }

    #[test]
    fn step_two_ousness() {
        assert_match(step_two, "callousness", "callous");
    }

    #[test]
    fn step_two_aliti() {
        assert_match(step_two, "formaliti", "formal");
    }

    #[test]
    fn step_two_iviti() {
        assert_match(step_two, "sensitiviti", "sensitive");
    }

    #[test]
    fn step_two_biliti() {
        assert_match(step_two, "sensibiliti", "sensible");
    }

    #[test]
    fn step_three_icate() {
        assert_match(step_three, "triplicate", "triplic");
    }

    #[test]
    fn step_three_ative() {
        assert_match(step_three, "formative", "form");
    }

    #[test]
    fn step_three_alize() {
        assert_match(step_three, "formalize", "formal");
    }

    #[test]
    fn step_three_iciti() {
        assert_match(step_three, "electriciti", "electric");
    }

    #[test]
    fn step_three_ical() {
        assert_match(step_three, "electrical", "electric");
    }

    #[test]
    fn step_three_ful() {
        assert_match(step_three, "hopeful", "hope");
    }

    #[test]
    fn step_three_ness() {
        assert_match(step_three, "goodness", "good");
    }

    #[test]
    fn step_four_al() {
        assert_match(step_four, "revival", "reviv");
    }

    #[test]
    fn step_four_ance() {
        assert_match(step_four, "allowance", "allow");
    }

    #[test]
    fn step_four_ence() {
        assert_match(step_four, "inference", "infer");
    }

    #[test]
    fn step_four_er() {
        assert_match(step_four, "airliner", "airlin");
    }

    #[test]
    fn step_four_ic() {
        assert_match(step_four, "gyroscopic", "gyroscop");
    }

    #[test]
    fn step_four_able() {
        assert_match(step_four, "adjustable", "adjust");
    }

    #[test]
    fn step_four_ible() {
        assert_match(step_four, "defensible", "defens");
    }

    #[test]
    fn step_four_ant() {
        assert_match(step_four, "irritant", "irrit");
    }

    #[test]
    fn step_four_ement() {
        assert_match(step_four, "replacement", "replac");
    }

    #[test]
    fn step_four_ment() {
        assert_match(step_four, "adjustment", "adjust");
    }

    #[test]
    fn step_four_ent() {
        assert_match(step_four, "dependent", "depend");
    }

    #[test]
    fn step_four_ion() {
        assert_match(step_four, "adoption", "adopt");
    }

    #[test]
    fn step_four_ou() {
        assert_match(step_four, "homologou", "homolog");
    }

    #[test]
    fn step_four_ism() {
        assert_match(step_four, "communism", "commun");
    }

    #[test]
    fn step_four_ate() {
        assert_match(step_four, "activate", "activ");
    }

    #[test]
    fn step_four_iti() {
        assert_match(step_four, "angulariti", "angular");
    }

    #[test]
    fn step_four_ous() {
        assert_match(step_four, "homologous", "homolog");
    }

    #[test]
    fn step_four_ive() {
        assert_match(step_four, "effective", "effect");
    }

    #[test]
    fn step_four_ize() {
        assert_match(step_four, "bowdlerize", "bowdler");
    }

    #[test]
    fn step_five_a_e_m_zero_no_update() {
        assert_match(step_five_a, "rate", "rate");
    }

    #[test]
    fn step_five_a_e_m_more_than_one_update() {
        assert_match(step_five_a, "probate", "probat");
    }

    #[test]
    fn step_five_a_e_m_one_update() {
        assert_match(step_five_a, "cease", "ceas");
    }

    #[test]
    fn step_five_b_update() {
        assert_match(step_five_b, "controll", "control");
    }

    #[test]
    fn step_five_b_no_update() {
        assert_match(step_five_b, "roll", "roll");
    }

}
