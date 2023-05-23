use std::fs;
use std::io::{self, BufRead};

fn get_words(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split('\n')
        .filter(|word| word.len() > 0)
        .map(|word| word.trim())
        .map(|word| word.to_string().to_lowercase())
        .collect()
}

fn get_guess() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    if stdin.lock().read_line(&mut line).is_err() {
        return get_guess();
    }
    line.trim().to_string().to_lowercase()
}

fn sort_to_vec(word: &String) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    chars
}

fn is_substring(real_word: &String, guess: &String) -> bool {
    let real_word = sort_to_vec(&real_word);
    let guess = sort_to_vec(&guess);
    let mut guess_i: usize = 0;

    for real_word_c in real_word {
        match guess.get(guess_i) {
            Some(guess_c) => {
                if *guess_c == real_word_c {
                    guess_i += 1;
                }
            }
            None => {
                return true;
            }
        }
    }
    return false;
}

fn get_word_matches(guess: &String, words: &Vec<String>, count: usize) -> Vec<String> {
    words
        .iter()
        .filter(|word| is_substring(word, &guess))
        .take(count)
        .map(|word| word.to_string())
        .collect()
}

fn main() {
    let words = get_words("words.txt".to_string());
    let count = 5;

    loop {
        let guess = get_guess();
        let matches = get_word_matches(&guess, &words, count);

        println!("Showing {} of max {count} matches", matches.len());
        for word in matches {
            println!("- {}", word);
        }
    }
}
