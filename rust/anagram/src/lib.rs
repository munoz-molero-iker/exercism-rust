use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {

    let mut valid_anagrams: HashSet<&'a str> = HashSet::new();
    
    for &possible_anagram in possible_anagrams {

        if possible_anagram.len() != word.len() { continue; }
        if sort_string(possible_anagram) != sort_string(word) { continue; }
        if check_equality(word, possible_anagram) { continue; }
        valid_anagrams.insert(possible_anagram);

    }

    valid_anagrams

}

fn sort_string (word: &str) -> String {

    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    chars.into_iter().collect()

}

fn check_equality (word_a: &str, word_b: &str) -> bool { word_a.to_lowercase() == word_b.to_lowercase() }