use std::fs;
use std::io::{self, BufRead, Write};

fn get_words(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split('\n')
        .filter(|word| word.len() > 1)
        .map(|word| word.trim())
        .map(|word| word.to_string().to_lowercase())
        .collect()
}

fn get_guess() -> String {
    let mut line = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
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
    let mut guess = sort_to_vec(&guess);

    for real_word_c in real_word {
        if guess.len() == 0 {
            return true;
        }
        if real_word_c == guess[0] {
            guess.remove(0);
        }
    }
    guess.len() == 0
}

fn get_word_matches(guess: &String, words: &Vec<String>, count: usize) -> Vec<String> {
    words
        .iter()
        .filter(|word| word.len() <= guess.len())
        .filter(|word| is_substring(word, &guess))
        .take(count)
        .map(|word| word.to_string())
        .collect()
}

fn main() {
    let words = get_words("words.txt".to_string());
    println!("Loaded {} words", words.len());
    let count = 10;

    loop {
        let guess = get_guess();
        let matches = get_word_matches(&guess, &words, count);

        println!("Showing {} of max {count} matches", matches.len());
        for word in matches {
            println!("- {}", word);
        }
    }
}
