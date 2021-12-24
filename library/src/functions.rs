use crate::definitions::*;
use anyhow::Result;
use colored::*;

use docx::{document::BodyContent, DocxFile};
use regex::Regex;
use std::borrow::Cow;
use std::{fs, path::PathBuf};

pub fn split_text_into_words(s: String) -> Result<Vec<Word>, TonalDistanceError> {
    // let's snag some words
    let re = Regex::new(r"(\w[\w']*)[\W]*");
    let re = match re {
        Ok(r) => r,
        Err(e) => return Err(TonalDistanceError::RegexError { source: e }),
    };

    // let's track paragraph for fun
    let mut paragraph_count: u32 = 0;

    let split_words = re
        .captures_iter(&s)
        .map(|preword| {
            // 0 is the whole capture, 1 is the first bracket.
            let original_word = preword.get(0).expect("No capture found").as_str();
            let pre_pure_word = preword.get(1).expect("No capture found").as_str();

            // we want to iterate paragraph count without borrowing this later, but we also want to be accurate about current paragraph.
            let mut accounting = 0;
            if original_word.contains("\n") {
                paragraph_count += 1;
                accounting = 1;
            }

            (
                String::from(pre_pure_word.to_lowercase()),
                original_word,
                paragraph_count - accounting,
            )
        })
        .enumerate()
        .map(|(j, tupl)| Word {
            pure_word: String::from(tupl.0),
            original_word: String::from(tupl.1),
            repeated: false,
            paragraph: tupl.2,
            word_position: j as u32,
        })
        .collect::<Vec<Word>>();

    Ok(split_words)
}

pub fn mark_up(v: Vec<Word>, stop_words: Vec<String>, buffer_length: usize) -> Vec<Word> {
    let mut matches: Vec<u32> = vec![];

    v.clone()
        .into_iter()
        .enumerate()
        .map(|(i, word)| {
            if stop_words.contains(&word.pure_word) {
                return word;
            }

            // don't scan beyond the end of the vec
            let end = if i + buffer_length + 1 > v.len() {
                v.len()
            } else {
                i + buffer_length + 1
            };

            let match_index = v[i + 1..end]
                .into_iter()
                .position(|x| x.pure_word == word.pure_word);

            match match_index {
                Some(matching_index) => {
                    matches.push((1 + i + matching_index) as u32);
                    Word {
                        repeated: true,
                        ..word
                    }
                }
                None => {
                    // if they're an ending word, they still get caught
                    if matches.contains(&word.word_position) {
                        Word {
                            repeated: true,
                            ..word
                        }
                    } else {
                        word
                    }
                }
            }
        })
        .collect::<Vec<Word>>()
}

pub fn report(v: &Vec<Word>) -> String {
    let f = format!(
        "{}",
        v.iter()
            .filter(|word| word.repeated)
            .map(|word| word.represent())
            .collect::<Vec<String>>()
            .join("\n")
    );

    f
}

pub fn rebuild_run(v: Vec<Word>) -> Vec<Run> {
    let mut run_vec: Vec<Run> = vec![];

    for word in v.iter() {
        if run_vec.len() > 0 && word.repeated == run_vec.last().unwrap().repeated {
            run_vec
                .last_mut()
                .unwrap()
                .text
                .push_str(&word.original_word)
        } else {
            run_vec.push(Run {
                text: word.original_word.to_owned(),
                repeated: word.repeated,
            })
        }
    }

    return run_vec;
}

pub fn colorize_run(v: Vec<Run>) -> String {
    let mut s = String::from("");

    for r in v.iter() {
        if r.repeated {
            s.push_str(&r.text.red().bold());
        } else {
            s.push_str(&r.text);
        }
    }
    s
}

pub fn parse_doc(path: PathBuf) -> Result<String, TonalDistanceError> {
    let docx = DocxFile::from_file(path).unwrap();
    let doc = docx.parse().unwrap();
    let mut paragraphs: Vec<Cow<str>> = vec![];
    for body_content in doc.document.body.iter() {
        match body_content {
            BodyContent::Paragraph(stuff) => paragraphs.push(
                stuff
                    .iter_text()
                    .map(|cow| cow.as_ref().to_string())
                    .collect(),
            ),
            BodyContent::Table(_) => println!("naw?"),
        }
    }
    Ok(paragraphs.join("\n"))
}

pub fn get_content_from_file(pb: PathBuf) -> Result<String, TonalDistanceError> {
    let ext = pb.extension().expect("Please specify the file extension");

    let content = if ext == "docx" {
        parse_doc(pb.clone())?
    } else {
        std::fs::read_to_string(pb)?
    };

    Ok(content)
}

pub fn get_stop_words(pre_stop_words: Option<Source>) -> Vec<String> {
    if let Some(existing) = pre_stop_words {
        println!("IF!");

        match existing {
            // stop words are in a file
            Source::Pb(src) => {
                let stop_words_string =
                    fs::read_to_string(src).expect("Could not read the stop words file");

                stop_words_string
                    .split("\n")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            }

            // stop words are a string
            Source::Raw(src) => src
                .split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        }
    } else {
        println!("ELSE!");
        let pre_vec =
            fs::read_to_string("./stop_words.txt").expect("Could not read the stop words file");

        pre_vec
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
    }
}

pub fn tell_you_how_bad(
    s: String,
    buffer_length: usize,
    stop_words: Vec<String>,
    response_type: ResponseType,
) -> Result<Response, TonalDistanceError> {
    let word_vec = split_text_into_words(s)?;

    // mark up the structs.
    let marked_up_vec: Vec<Word> = mark_up(word_vec, stop_words, buffer_length);

    // create report.
    let response: Response = match response_type {
        ResponseType::Raw => Response::VecOfRuns(rebuild_run(marked_up_vec)),
        // ResponseType::Colorized => library::rebuild(marked_up_vec, true),
        ResponseType::Formatted => Response::Str(report(&marked_up_vec)),
    };

    Ok(response)
}
