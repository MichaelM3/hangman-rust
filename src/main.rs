use rand::{thread_rng, Rng};
use std::io;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let words = vec!["hello", "goodbye", "rust", "programming", "something"];
    let rand_index = thread_rng().gen_range(0..words.len());
    
    let word = words[rand_index];
    let mut letters = create_letters(&word);

    // let mut guesses: Vec<char> = Vec::new();
    let mut lives: u8 = 6;

    println!("{}", word);
    
    loop {
        println!("You have {} lives remaining.", lives);
        display_progress(&letters);

        println!("Guess a letter");
        let guess = user_input();

        if guess == '*' {
            break;
        }

        let mut matched = false;
        for letter in letters.iter_mut() {
            if letter.character == guess {
                letter.revealed = true;
                matched = true; 
            }
        }

        if !matched {
            lives -= 1;
        }

        match check_progress(lives, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats you win! The word was {}", word);
                break;
            },
            GameProgress::Lost => {
                println!("Sorry you lost!");
                break;
            }
        }

    }

}

fn create_letters(word: &str) -> Vec<Letter> {
    // Creates empty vector
    let mut letters: Vec<Letter> = Vec::new();  

    // Wraps each character in a letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;

}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn user_input() -> char {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            match guess.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(lives: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut word_finished = true;
    for letter in letters {
        if !letter.revealed {
            word_finished = false;
        }
    }

    if word_finished {
        return GameProgress::Won;
    }

    if lives > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
