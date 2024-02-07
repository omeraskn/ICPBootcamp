struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(&self) -> String {
        self.clone()
    }
    
    fn count_words(&self) -> usize {
        let words: Vec<&str> = self.text.split_whitespace().collect();          // Collect every word in a vector using .collect() than use that vectors length.
        
        words.len()
    }
}

fn main() {
    println!("Enter a text to calculate how many words there is in the text:");
    
    let mut read_line = String::new();
    std::io::stdin().read_line(&mut read_line).unwrap();            // Read text from user
    
    let word_counter = WordCounter::new(input_text.trim());         // Put the text inside of the struct
    
    let result = if word_counter.text.is_empty() {
        Err("Error: Entered text is empty.")
    } else {
        let word_count = word_counter.count_words();                // Call the word count function
        Ok(word_count)      
    };

    match result {
        Ok(word_count) => println!("Word count: {}", word_count),
        Err(error) => println!("{}", error),
    }
}