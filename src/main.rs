mod wordlist;
use wordlist::*;

fn main() {
    let mut all_words = Wordlist::load("wordlist").expect("Unable to load words");
    println!("Loaded {} words", all_words.len());
    all_words.print_letter_frequencies();
    while all_words.len() > 1 {
        if all_words.len() == 0 {
            panic!("Woah I don't know that word");
        }
        let guess = all_words.words_by_eliminate().next().unwrap();
        println!("I guess {}", guess);
        println!("How did I do?  Tell me with:");
        println!("Not present: -");
        println!("present: +");
        println!("exact: =");
        let mark = loop {
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Unable to read stdin?");
            let buf = buf.trim();
            if buf.len() != 5 || !buf.chars().all(|c| ['-', '+', '='].contains(&c)) {
                println!("For some reason I don't believe you, try again.");
            } else {
                break buf.to_owned();
            }
        };
        for (pos, res) in mark.chars().enumerate() {
            match res {
                '-' => all_words.eliminate_char(guess[pos]),
                '=' => all_words.eliminate_non_exact(pos, guess[pos]),
                '+' => all_words.eliminate_exact(pos, guess[pos]),
                _ => unreachable!(),
            }
        }
    }
    println!(
        "OK, I assert that the answer is: {}",
        all_words.words_by_value().last().unwrap()
    );
}
