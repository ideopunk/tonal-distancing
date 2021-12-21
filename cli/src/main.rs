use anyhow::{bail, Context, Result};
use library;
use std::io::{self, Write};
use std::str::FromStr;
// use std::time::Instant;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug)]
enum ResponseType {
    Raw,
    // Colorized,
    Report,
}

impl FromStr for ResponseType {
    type Err = anyhow::Error;

    fn from_str(res_type: &str) -> Result<Self, anyhow::Error> {
        match res_type {
            "raw" => Ok(ResponseType::Raw),
            // "colorized" => Ok(ResponseType::Colorized),
            "report" => Ok(ResponseType::Report),
            _ => bail!("Could not parse a response type"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tonal-distancing", about = "Look for repeated words")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Set how far ahead to check
    #[structopt(
        short = "l",
        long = "lookahead",
        default_value = "50",
        name = "Buffer Length"
    )]
    buffer_length: u32,

    // Optional personal stop-word list
    #[structopt(short = "s", long = "stopwords", name = "Stop Words")]
    stop_words: Option<PathBuf>,

    // Optional output specification
    #[structopt(
        short = "r",
        long = "response",
        name = "Response type",
        default_value = "raw",
        case_insensitive = true
    )]
    response: ResponseType,
}

fn main() -> Result<()> {
    // let now = Instant::now();

    let args = Cli::from_args();
    // get our words.

    let ext = args
        .path
        .extension()
        .expect("Please specify the file extension");
    // let content = if &args.path

    let content = if ext == "docx" {
        library::parse_doc(args.path.clone()).context(format!(
            "Failed to read the contents of {} to string",
            args.path.to_str().unwrap()
        ))?
    } else {
        std::fs::read_to_string(&args.path).context(format!(
            "Failed to read the contents of {} to string",
            args.path.to_str().unwrap()
        ))?
        // std::fs::read_to_string(&args.path).expect("Could not read input file")?
    };

    let word_vec = library::split_text_into_words(content)?;

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
    let marked_up_vec: Vec<library::Word> =
        library::mark_up(word_vec, stop_words, args.buffer_length as usize);

    // create report.
    let res: String = match args.response {
        ResponseType::Raw => library::rebuild(marked_up_vec, false),
        // ResponseType::Colorized => library::rebuild(marked_up_vec, true),
        ResponseType::Report => library::report(&marked_up_vec),
    };

    // write report to file
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", res);

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
