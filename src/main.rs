use std::{fs, path::PathBuf};
use structopt::StructOpt;
use regex::Regex;
use itertools;
use std::time::Instant;

#[derive(StructOpt, Debug)]
#[structopt(name="tonal-distancing", about = "Look for repeated words")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Set how far ahead to check
    #[structopt(short = "l", long = "lookahead", default_value = "50")]
    buffer_length: u32,

    // Optional personal stop-word list
    #[structopt(short = "s", long = "stopwords")]
    stop_words: Option<PathBuf>
}

#[derive(Debug, PartialEq, Clone)]
struct Word {
    text: String,
    repeated: bool,
    word_position: u32,
    paragraph: u32
}

impl Word {
    fn represent(&self) -> String {
        format!("Word: {},\t\tParagraph: {},\t\tWord Position: {}", self.text, self.paragraph + 1, self.word_position + 1)
    }
}

fn split_text_into_words(s: String) -> Vec<Word> {
    let seperator = Regex::new(r"([ !',.\n]+)").expect("Invalid regex");

  // get all our words
    let paragraphs_of_word_arrays: Vec<Vec<Word>> = s.lines()
            .enumerate().map(|(i, line)| seperator.split(line) // i is used later to indicate paragraph that owns the word. 
            .into_iter().enumerate().map(|(j, word)| Word {text: String::from(word), repeated: false, paragraph: i as u32, word_position: j as u32})
            .collect::<Vec<Word>>()).collect();

    itertools::concat(paragraphs_of_word_arrays)
}

fn mark_up(v: Vec<Word>, stop_words: Vec<&str>, buffer_length: usize) -> Vec<Word> {
    v.clone().into_iter().enumerate().map(|(i, word)| {
        if stop_words.contains(&&word.text.to_lowercase().as_ref()) {
            return word
        }

        let end = if i + buffer_length > v.len() {
            v.len()
        } else {
            i + buffer_length 
        };

        if v[i+1..end].into_iter().any(|x| x.text == word.text) {
            return Word {text: word.text, repeated: true, paragraph: word.paragraph, word_position: word.word_position}
        }
        return word
    }).collect::<Vec<Word>>()
}

fn main() {
    let now = Instant::now();
    let args = Cli::from_args();

    // get our words.
    let content = std::fs::read_to_string(&args.path).expect("Could not read input file");
    let word_vec = split_text_into_words(content);
    
    // get our stop words
    let stop_words_string = match &args.stop_words {
        Some(file) => fs::read_to_string(file).expect("Could not read the stop words file"),
        None => fs::read_to_string("stop_words.txt").expect("Could not read the stop words file")
    };
    let stop_words = stop_words_string.split("\n").collect::<Vec<&str>>();


    // initialize the report
    let _ = fs::File::create("report.txt").expect("Failed to create report file.");
    // let mut report = String::new();

    // mark up the structs. 
    let marked_up_vec: Vec<Word> = mark_up(word_vec, stop_words, args.buffer_length as usize);
    
    // create report.
    let report = format!("{}",  
        marked_up_vec.iter().filter(|word| word.repeated).map(|word| word.represent()).collect::<Vec<String>>().join("\n")
    );

    // write report to file
    let _ = fs::write("report.txt", report);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}