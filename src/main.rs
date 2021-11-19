use std::{fmt::format, fs, path::PathBuf};
use structopt::StructOpt;
use regex::Regex;
use itertools;

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
    paragraph: u32
}

impl Word {
    fn represent(&self) -> String {
        format!("Word: {},\t\t Paragraph: {}", self.text, self.paragraph)
    }
}

fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).expect("Could not read input file");
    let vec = content.lines();

    let seperator = Regex::new(r"([ !',.\n]+)").expect("Invalid regex");

    
    // get all our words
    let paragraphs_of_word_arrays: Vec<Vec<Word>> = vec
            .enumerate().map(|(i, line)| seperator.split(line)
            .into_iter().map(|word| Word {text: String::from(word), repeated: false, paragraph: i as u32})
            .collect::<Vec<Word>>()).collect();

    let word_vec = itertools::concat(paragraphs_of_word_arrays);
    
    // get our stop words
    let stop_words_string = match &args.stop_words {
        Some(file) => fs::read_to_string(file).expect("Could not read the stop words file"),
        None => fs::read_to_string("stop_words.txt").expect("Could not read the stop words file")
    };
    let stop_words = stop_words_string.split("\n").collect::<Vec<&str>>();


    // initialize the report
    let _ = fs::File::create("report.txt").expect("Failed to create report file.");
    // let mut report = String::new();

    // mark up the vector
    let marked_up_vec: Vec<Word> = word_vec.clone().into_iter().enumerate().map(|(i, word)| {
        if stop_words.contains(&&word.text.to_lowercase().as_ref()) {
            return word
        }

        // println!("{}", args.buffer_length as usize);
        let end = if i + args.buffer_length as usize > word_vec.len() {
            word_vec.len()
        } else {
            i + args.buffer_length as usize
        };

        println!("{}", end);

        if word_vec[i+1..end].into_iter().any(|x| x.text == word.text) {
            return Word {text: word.text, repeated: true, paragraph: word.paragraph}
        }
        return word
    }).collect::<Vec<Word>>();

    let report = format!(
        "{}",  marked_up_vec.iter().filter(|word| word.repeated).map(|word| word.represent()).collect::<Vec<String>>().join("\n")
    );

    println!("{}", report);

    let _ = fs::write("report.txt", report);
}