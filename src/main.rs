use std::{fs, hash::Hasher, io::Write};
use structopt::StructOpt;
use regex::{Regex, bytes};

/// Get a file to parse.
#[derive(StructOpt, Debug)]
#[structopt(about = "Create a Typescript file")]
struct Cli {
    /// Name of file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

static STOP_WORDS: [&str; 184] = ["a",
"about",
"above",
"actually",
"after",
"again",
"against",
"all",
"almost",
"also",
"although",
"always",
"am",
"an",
"and",
"any",
"are",
"as",
"at",
"be",
"became",
"become",
"because",
"been",
"before",
"being",
"below",
"between",
"both",
"but",
"by",
"can",
"could",
"did",
"do",
"does",
"doing",
"down",
"during",
"each",
"either",
"else",
"few",
"for",
"from",
"further",
"had",
"has",
"have",
"having",
"he",
"he'd",
"he'll",
"hence",
"he's",
"her",
"here",
"here's",
"hers",
"herself",
"him",
"himself",
"his",
"how",
"how's",
"I",
"I'd",
"I'll",
"I'm",
"I've",
"if",
"in",
"into",
"is",
"it",
"it's",
"its",
"itself",
"just",
"let's",
"may",
"maybe",
"me",
"might",
"mine",
"more",
"most",
"must",
"my",
"myself",
"neither",
"nor",
"not",
"of",
"oh",
"on",
"once",
"only",
"ok",
"or",
"other",
"ought",
"our",
"ours",
"ourselves",
"out",
"over",
"own",
"same",
"she",
"she'd",
"she'll",
"she's",
"should",
"so",
"some",
"such",
"than",
"that",
"that's",
"the",
"their",
"theirs",
"them",
"themselves",
"then",
"there",
"there's",
"these",
"they",
"they'd",
"they'll",
"they're",
"they've",
"this",
"those",
"through",
"to",
"too",
"under",
"until",
"up",
"very",
"was",
"we",
"we'd",
"we'll",
"we're",
"we've",
"were",
"what",
"what's",
"when",
"whenever",
"when's",
"where",
"whereas",
"wherever",
"where's",
"whether",
"which",
"while",
"who",
"whoever",
"who's",
"whose",
"whom",
"why",
"why's",
"will",
"with",
"within",
"would",
"yes",
"yet",
"you",
"you'd",
"you'll",
"you're",
"you've",
"your",
"yours",
"yourself",
"yourselves"];

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
            if STOP_WORDS.contains(&word) {
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