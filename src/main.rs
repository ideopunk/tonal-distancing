use std::{fs};
use structopt::StructOpt;
use regex::{Regex};

mod stop_words;

/// Get a file to parse.
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Typescript file")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    println!("Hello, world!");
    let args = Cli::from_args();
    println!("{:?}", args);

    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    let vec = content.lines();

    let seperator = Regex::new(r"([ ,.]+)").expect("Invalid regex");
    let paragraphs_of_word_arrays: Vec<Vec<&str>> = vec.map(|line| seperator.split(line).into_iter().collect::<Vec<&str>>()).collect();

    let _ = fs::File::create("report.txt").expect("Failed to create report file.");

    let mut report = String::new();

    for paragraph in paragraphs_of_word_arrays {
        for (i, &word) in paragraph.iter().enumerate() {
            // move on if this is 'a name'
            if word.chars().nth(0).unwrap().is_uppercase() {
                continue
            }

            // move on if this is a 'stop word'
            if stop_words::STOP_WORDS.contains(&word) {
                continue;
            }

            let result = paragraph[i+1..].iter().position(|&x|   x == word);
            let _ =  match result {
                Some(res) => {
                    println!("{}", res);
                    let frm = format!("\n{}\n{}", word, &res.to_string());
                    report.push_str(&frm);
                },
                None => continue
            };
        }
    }

    let _ = fs::write("report.txt", report);
}

// need to iter fooooorward. 