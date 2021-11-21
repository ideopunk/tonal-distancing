use itertools;
use regex::Regex;
use std::time::Instant;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tonal-distancing", about = "Look for repeated words")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Set how far ahead to check
    #[structopt(short = "l", long = "lookahead", default_value = "50")]
    buffer_length: u32,

    // Optional personal stop-word list
    #[structopt(short = "s", long = "stopwords")]
    stop_words: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Clone)]
struct Word {
    text: String,
    repeated: bool,
    word_position: u32,
    paragraph: u32,
}

impl Word {
    fn represent(&self) -> String {
        let word_buff = vec![' '; 20 - self.text.len()]
            .into_iter()
            .collect::<String>();

        let paragraph_buff = vec![' '; 10 - (self.paragraph + 1).to_string().len()]
            .into_iter()
            .collect::<String>();
        format!(
            "Word: {}{}Paragraph: {}{}Word Position: {}",
            self.text,
            word_buff,
            self.paragraph + 1,
            paragraph_buff,
            self.word_position + 1
        )
    }
}

fn split_text_into_words(s: String) -> Vec<Word> {
    let seperator = Regex::new(r"([ !',.\n]+)").expect("Invalid regex");

    let re = Regex::new(r"([\w']+)").unwrap();

    // get all our words
    let paragraphs_of_word_arrays: Vec<Vec<Word>> = s
        .lines()
        // i is used later to indicate paragraph that owns the word.
        .enumerate()
        .map(|(i, line)| {
            seperator
                .split(line)
                .into_iter()
                .filter_map(|word| {
                    let trimmed_word = &re.captures_iter(word).next();
                    match trimmed_word {
                        Some(trimmed) => Some(trimmed[0].to_string()),
                        None => None,
                    }
                })
                .enumerate()
                .map(|(j, text)| Word {
                    text,
                    repeated: false,
                    paragraph: i as u32,
                    word_position: j as u32,
                })
                .collect::<Vec<Word>>()
        })
        .collect();

    itertools::concat(paragraphs_of_word_arrays)
}

fn mark_up(v: Vec<Word>, stop_words: Vec<&str>, buffer_length: usize) -> Vec<Word> {
    v.clone()
        .into_iter()
        .enumerate()
        .map(|(i, word)| {
            let lowercase_word = word.text.to_lowercase();
            if stop_words.contains(&&lowercase_word.as_ref()) {
                return word;
            }

            let end = if i + buffer_length + 1 > v.len() {
                v.len()
            } else {
                i + buffer_length + 1
            };

            if v[i + 1..end]
                .into_iter()
                .any(|x| x.text.to_lowercase() == lowercase_word)
            {
                println!("{} {}, {}", word.represent(), i + 1, end);
                return Word {
                    text: word.text,
                    repeated: true,
                    paragraph: word.paragraph,
                    word_position: word.word_position,
                };
            }

            word
        })
        .collect::<Vec<Word>>()
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
        None => fs::read_to_string("stop_words.txt").expect("Could not read the stop words file"),
    };
    let stop_words = stop_words_string.split("\n").collect::<Vec<&str>>();

    // initialize the report
    let _ = fs::File::create("report.txt").expect("Failed to create report file.");
    // let mut report = String::new();

    // mark up the structs.
    let marked_up_vec: Vec<Word> = mark_up(word_vec, stop_words, args.buffer_length as usize);

    // create report.
    let report = format!(
        "{}",
        marked_up_vec
            .iter()
            .filter(|word| word.repeated)
            .map(|word| word.represent())
            .collect::<Vec<String>>()
            .join("\n")
    );

    // write report to file
    let _ = fs::write("report.txt", report);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
