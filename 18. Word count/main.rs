use std::io::{self,Write};

fn count_words(sentence:&str)->usize{
    sentence.split_whitespace().filter(|word| !word.is_empty()).count()
}
fn main() {
    let s1 = "Hello, world!";
    println!("word count: {}",count_words(s1));
}
