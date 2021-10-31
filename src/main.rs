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
    println!("Hello, world!");
    let args = Cli::from_args();
    println!("{:?}", args);

    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    let vec = content.lines();

    let seperator = Regex::new(r"([ ,.]+)").expect("Invalid regex");
    let paragraphs_of_word_arrays: Vec<Vec<&str>> = vec.map(|line| seperator.split(line).into_iter().collect::<Vec<&str>>()).collect();

    
    println!("{:?}", paragraphs_of_word_arrays);
}
