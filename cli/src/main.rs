use library;
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
}

fn main() {
    let now = Instant::now();

    let args = Cli::from_args();
    // get our words.

    let ext = args
        .path
        .extension()
        .expect("Please specify the file extension");
    // let content = if &args.path

    let content = if ext == "docx" {
        library::parse_doc(args.path)
    } else {
        std::fs::read_to_string(&args.path).expect("Could not read input file")
    };
    let word_vec = library::split_text_into_words(content);

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
    let report = library::report(&marked_up_vec);

    let marked_up_content = library::rebuild(marked_up_vec);

    let total_report = format!("{}\n\n\n{}", report, marked_up_content);
    
    // write report to file
    let _ = fs::write("report.txt", total_report);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
