// use docx::{document::BodyContent, DocxFile};
use docx_rs;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    pub pure_word: String,     // sub
    pub original_word: String, // Sub!"
    pub repeated: bool,
    pub word_position: u32,
    pub paragraph: u32,
}

impl Word {
    pub fn represent(&self) -> String {
        let word_buff = vec![' '; 20 - self.pure_word.len()]
            .into_iter()
            .collect::<String>();

        let paragraph_buff = vec![' '; 10 - (self.paragraph + 1).to_string().len()]
            .into_iter()
            .collect::<String>();
        format!(
            "Word: {}{}Paragraph: {}{}Word Position: {}",
            self.pure_word,
            word_buff,
            self.paragraph + 1,
            paragraph_buff,
            self.word_position + 1
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Run {
    pub text: String,
    pub repeated: bool,
}

///TonalDistanceError enumerates all possible errors returneed by this lib.
#[derive(Error, Debug)]
pub enum TonalDistanceError {
    /// Represents a regex error.
    #[error("Regex error")]
    RegexError { source: regex::Error },

    /// Represents a failure to read a docx file.
    #[error("Failed to read from docx file")]
    DocXReadError {
        #[from]
        source: docx_rs::ReaderError,
    },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub fn split_text_into_words(s: String) -> Result<Vec<Word>, TonalDistanceError> {
    // let's snag some words
    let re = Regex::new(r"(\w[\w']*)[ \r\n-]*");
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

            // uhh we want to iterate paragraph count without borrowing this later, but we also want to be accurate about current paragraph.
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

pub fn mark_up(v: Vec<Word>, stop_words: Vec<&str>, buffer_length: usize) -> Vec<Word> {
    let mut matches: Vec<u32> = vec![];

    v.clone()
        .into_iter()
        .enumerate()
        .map(|(i, word)| {
            if stop_words.contains(&word.pure_word.as_ref()) {
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
            // if v[i + 1..end]
            //     .into_iter()
            //     .any(|x| x.text.to_lowercase() == lowercase_word)
            // {
            //     return Word {
            //         text: word.text,
            //         repeated: true,
            //         paragraph: word.paragraph,
            //         word_position: word.word_position,
            //     };
            // }

            // word
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

pub fn rebuild(v: Vec<Word>, _: bool) -> String {
    v.iter().fold(String::from(""), |mut acc, word| {
        if word.repeated {
            acc.push_str("#")
        };
        acc.push_str(&word.original_word);
        acc
    })
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

pub fn parse_doc(path: PathBuf) -> Result<String, TonalDistanceError> {
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    // let mut file = File::create("./test.json").unwrap();
    let res = docx_rs::read_docx(&buf);

    match res {
        Ok(result) => Ok(result.json()),
        Err(source) => Err(TonalDistanceError::DocXReadError { source }),
    }

    // Ok(res.json)

    // let mut  paragraphs = res.document.children.map(child);
    // let docx = docx_rs::from_file(path)?;
    // let doc = docx.parse()?;
    // let mut paragraphs: Vec<Cow<str>> = vec![];
    // for body_content in res.document.children.iter() {
    //     match body_content {
    //         docx_rs::DocumentChild::Paragraph(stuff) => paragraphs.push(
    //             stuff
    //                 .children()
    //                 .iter()
    //                 .map(|par_chil| match par_chil {
    //                     docx_rs::ParagraphChild::Run(stuff) => println!("{}", stuff),
    //                     _ => println!("naw!"),
    //                 })
    //                 .collect(),
    //         ),
    //         _ => println!("naw?"),
    //     }
    // }
    // Ok(paragraphs.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions;

    #[test]
    fn test_splitting_text_into_words() -> Result<(), TonalDistanceError> {
        let word_vec = split_text_into_words(String::from("here\nI'm here-\nthe snow falling"))?;
        pretty_assertions::assert_eq!(
            word_vec,
            vec![
                Word {
                    pure_word: String::from("here"),
                    paragraph: 0,
                    repeated: false,
                    original_word: String::from("here\n"),
                    word_position: 0
                },
                Word {
                    pure_word: String::from("i'm"),
                    paragraph: 1,
                    repeated: false,
                    original_word: String::from("I'm "),
                    word_position: 1
                },
                Word {
                    pure_word: String::from("here"),
                    paragraph: 1,
                    repeated: false,
                    original_word: String::from("here-\n"),
                    word_position: 2
                },
                Word {
                    pure_word: String::from("the"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("the "),
                    word_position: 3
                },
                Word {
                    pure_word: String::from("snow"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("snow "),
                    word_position: 4
                },
                Word {
                    pure_word: String::from("falling"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("falling"),
                    word_position: 5
                },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_splitting_nothin() -> Result<(), TonalDistanceError> {
        let word_vec = split_text_into_words(String::from(""))?;
        pretty_assertions::assert_eq!(word_vec, vec![]);
        Ok(())
    }

    #[test]
    fn test_markup() {
        let original_vec = vec![
            Word {
                pure_word: String::from("here"),
                paragraph: 0,
                repeated: false,
                original_word: String::from("here\n"),
                word_position: 0,
            },
            Word {
                pure_word: String::from("i'm"),
                paragraph: 1,
                repeated: false,
                original_word: String::from("I'm "),
                word_position: 1,
            },
            Word {
                pure_word: String::from("here"),
                paragraph: 1,
                repeated: false,
                original_word: String::from("here-\n"),
                word_position: 2,
            },
            Word {
                pure_word: String::from("the"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("the "),
                word_position: 3,
            },
            Word {
                pure_word: String::from("snow"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("snow "),
                word_position: 4,
            },
            Word {
                pure_word: String::from("falling"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("falling"),
                word_position: 5,
            },
        ];

        let marked_up_vec = mark_up(original_vec, vec![], 10);

        pretty_assertions::assert_eq!(
            marked_up_vec,
            [
                Word {
                    pure_word: String::from("here"),
                    paragraph: 0,
                    repeated: true,
                    original_word: String::from("here\n"),
                    word_position: 0,
                },
                Word {
                    pure_word: String::from("i'm"),
                    paragraph: 1,
                    repeated: false,
                    original_word: String::from("I'm "),
                    word_position: 1,
                },
                Word {
                    pure_word: String::from("here"),
                    paragraph: 1,
                    repeated: true,
                    original_word: String::from("here-\n"),
                    word_position: 2,
                },
                Word {
                    pure_word: String::from("the"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("the "),
                    word_position: 3,
                },
                Word {
                    pure_word: String::from("snow"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("snow "),
                    word_position: 4,
                },
                Word {
                    pure_word: String::from("falling"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("falling"),
                    word_position: 5,
                },
            ]
        )
    }

    #[test]
    fn test_rebuilding() {
        let rebuilt_string = rebuild(
            vec![
                Word {
                    pure_word: String::from("here"),
                    paragraph: 0,
                    repeated: false,
                    original_word: String::from("here\n"),
                    word_position: 0,
                },
                Word {
                    pure_word: String::from("i'm"),
                    paragraph: 1,
                    repeated: false,
                    original_word: String::from("I'm "),
                    word_position: 1,
                },
                Word {
                    pure_word: String::from("here"),
                    paragraph: 1,
                    repeated: false,
                    original_word: String::from("here-\n"),
                    word_position: 2,
                },
                Word {
                    pure_word: String::from("the"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("the "),
                    word_position: 3,
                },
                Word {
                    pure_word: String::from("snow"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("snow "),
                    word_position: 4,
                },
                Word {
                    pure_word: String::from("falling"),
                    paragraph: 2,
                    repeated: false,
                    original_word: String::from("falling"),
                    word_position: 5,
                },
            ],
            false,
        );
        pretty_assertions::assert_eq!(rebuilt_string, "here\nI'm here-\nthe snow falling")
    }

    #[test]
    fn test_rebuild_a_run() {
        let rebuilt_run = rebuild_run(vec![
            Word {
                pure_word: String::from("here"),
                paragraph: 0,
                repeated: true,
                original_word: String::from("here\n"),
                word_position: 0,
            },
            Word {
                pure_word: String::from("i'm"),
                paragraph: 1,
                repeated: false,
                original_word: String::from("I'm "),
                word_position: 1,
            },
            Word {
                pure_word: String::from("here"),
                paragraph: 1,
                repeated: true,
                original_word: String::from("here-\n"),
                word_position: 2,
            },
            Word {
                pure_word: String::from("the"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("the "),
                word_position: 3,
            },
            Word {
                pure_word: String::from("snow"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("snow "),
                word_position: 4,
            },
            Word {
                pure_word: String::from("falling"),
                paragraph: 2,
                repeated: false,
                original_word: String::from("falling"),
                word_position: 5,
            },
        ]);
        pretty_assertions::assert_eq!(
            rebuilt_run,
            vec![
                Run {
                    text: String::from("here\n"),
                    repeated: true
                },
                Run {
                    text: String::from("I'm "),
                    repeated: false
                },
                Run {
                    text: String::from("here-\n"),
                    repeated: true
                },
                Run {
                    text: String::from("the snow falling"),
                    repeated: false
                }
            ]
        )
    }
}
