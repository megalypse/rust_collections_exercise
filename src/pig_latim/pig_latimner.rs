pub fn pig_latifier(word: String) -> String {
    let mut word = String::from(word);
    let hifen = String::from("-");
    let vowels = ["a", "e", "i", "o","u"];
    let first_letter: String = word.drain(0..1).collect();
    let first_letter: String = first_letter[0..1].to_lowercase();
    let word = String::from(word);
    let mut new_word = String::new();
    
    if vowels.contains(&&first_letter[0..1]) {
        let suffix = String::from("hay");
        new_word = String::from(word + &hifen + &first_letter + &suffix);
    } else {
        let suffix = String::from("ay");
        new_word = String::from(word + &hifen + &first_letter + &suffix);
    }

    new_word
}