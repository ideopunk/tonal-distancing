use docx::{document::BodyContent, DocxFile};
use itertools;
use regex::Regex;
use std::borrow::Cow;
use std::path::PathBuf;

#[cfg(test)]
use pretty_assertions::assert_eq;

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

pub fn split_text_into_words(s: String) -> Vec<Word> {
    // let's snag some words
    let re = Regex::new(r"(\w[\w']*)[ \r\n-]*").unwrap();

    // let's track paragraph for fun
    let mut paragraph_count: u32 = 0;

    re.captures_iter(&s)
        .map(|preword| {
            let original_word = preword.get(0).unwrap().as_str();
            let pre_pure_word = preword.get(1).unwrap().as_str();

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
        .collect::<Vec<Word>>()
    // })
    // .collect();

    // itertools::concat(paragraphs_of_word_arrays)
}

pub fn mark_up(v: Vec<Word>, stop_words: Vec<&str>, buffer_length: usize) -> Vec<Word> {
    let mut matches: Vec<u32> = vec![];

    v.clone()
        .into_iter()
        .enumerate()
        .map(|(i, word)| {
            println!("{}, {}, {:?}", word.represent(), word.pure_word, matches);

            if stop_words.contains(&word.pure_word.as_ref()) {
                return word;
            }

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

pub fn rebuild(v: Vec<Word>) -> String {
    v.iter().fold(String::from(""), |mut acc, word| {
        if word.repeated {
            acc.push_str("#")
        };
        acc.push_str(&word.original_word);
        acc
    })
}

pub fn parse_doc(path: PathBuf) -> String {
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
    paragraphs.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_splitting_text_into_words() {
        let word_vec = split_text_into_words(String::from("here\nI'm here-\nthe snow falling"));
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
        )
    }
}
