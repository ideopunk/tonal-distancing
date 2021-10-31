use std::fs;
use structopt::StructOpt;
use regex::Regex;


/// Get a file to parse.
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Typescript file")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).expect("Could not read input file");
    let vec = content.lines();

    let seperator = Regex::new(r"([ ,.]+)").expect("Invalid regex");
    let paragraphs_of_word_arrays: Vec<Vec<&str>> = vec.map(|line| seperator.split(line).into_iter().collect::<Vec<&str>>()).collect();
    
    
    let stop_words_string = fs::read_to_string("stop_words.txt").expect("Could not read stop words file");
    let stop_words = stop_words_string.split("\n").collect::<Vec<&str>>();

    let _ = fs::File::create("report.txt").expect("Failed to create report file.");
    let mut report = String::new();

    for mut paragraph in paragraphs_of_word_arrays {

        // the last element is whitespace junk, ignore
        paragraph.pop();

        for (i, &word) in paragraph.iter().enumerate() {

            // move on if this is a stop word, including ones added by the user
            if stop_words.contains(&word) {
                continue;
            }

            let result = paragraph[i+1..].iter().position(|&x|   x == word);
            let _ =  match result {
                Some(res) => {
                    let frm = format!("\n{}\n{}", word, &res.to_string());
                    report.push_str(&frm);
                },
                None => continue
            };
        }
    }

    let _ = fs::write("report.txt", report);
}