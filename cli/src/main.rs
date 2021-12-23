use anyhow::{Context, Result};
use library;
use std::io::{self, Write};
// use std::time::Instant;
use std::path::PathBuf;
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

    // Optional output specification
    #[structopt(
        short = "r",
        long = "response",
        name = "Response type",
        default_value = "report",
        case_insensitive = true
    )]
    response: library::ResponseType,
}

pub fn write_report(report: library::Response) -> () {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = writeln!(handle, "{:?}", report);
    ()
}

pub fn main() -> Result<()> {
    // let now = Instant::now();

    let args = Cli::from_args();

    // get our big ol string
    let content =
        library::get_content_from_file(args.path).context("Failed to get content from file")?;

    // get our stop words
    let stop_words = library::get_stop_words_from_file(&args.stop_words);

    // get our report
    let res = library::tell_you_how_bad(
        content,
        args.buffer_length as usize,
        stop_words,
        args.response,
    )
    .context("Failed to process content")?;

    // write report to stdout
    write_report(res);

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
