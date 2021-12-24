use anyhow::{Context, Result};
use library::{definitions, functions};
use std::io::{self, Write};
// use std::time::Instant;
use std::fs::metadata;
use std::path::PathBuf;
use structopt::StructOpt;

fn source_from_str(input: &str) -> definitions::Source {
    let pb = PathBuf::from(input);

    let check = metadata(&pb);

    match check {
        Ok(_) => definitions::Source::Pb(pb),
        Err(_) => definitions::Source::Raw(String::from(input)),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tonal-distancing", about = "Look for repeated words")]
struct Cli {
    /// Content to evaluate. Accepts a file path or a string.
    #[structopt(parse(from_str = source_from_str))]
    source: definitions::Source,

    /// Set how far ahead to check
    #[structopt(
        short = "l",
        long = "lookahead",
        default_value = "50",
        name = "Buffer Length"
    )]
    buffer_length: u32,

    /// Optional personal stop-word list. 
    /// Accepts a comma-separated list, or a file path to a line-separated list.
    /// If not provided, a default list is used.
    #[structopt(short = "s", long = "stopwords", name = "Stop Words", parse(from_str = source_from_str))]
    stop_words: Option<definitions::Source>,

    /// Optional output specification.
    /// [values: "raw" | "formatted"] 
    /// [default: "formatted"]
    #[structopt(
        short = "r",
        long = "response",
        name = "Response Type",
        // default_value = "report",
        case_insensitive = true
    )]
    response: Option<definitions::ResponseType>,
}

pub fn write_report(report: definitions::Response) -> () {
    match report {
        definitions::Response::Str(s) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let _ = writeln!(handle, "{}", s);
            ()
        }
        definitions::Response::VecOfRuns(v) => {
            // render!
            let colorized = functions::colorize_run(v);
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let _ = writeln!(handle, "{}", colorized);
            ()
        }
    };
    ()
}

pub fn main() -> Result<()> {
    // let now = Instant::now();

    let args = Cli::from_args();

    // get our big ol string
    let content = match args.source {
        definitions::Source::Pb(src) => {
            functions::get_content_from_file(src).context("Failed to get content from file")?
        }
        definitions::Source::Raw(src) => src,
    };

    // get our stop words
    let stop_words = functions::get_stop_words(args.stop_words);

    // get our report
    let res = functions::tell_you_how_bad(
        content,
        args.buffer_length as usize,
        stop_words,
        args.response
            .unwrap_or(definitions::ResponseType::Formatted),
    )
    .context("Failed to process content")?;

    // write report to stdout
    write_report(res);

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

