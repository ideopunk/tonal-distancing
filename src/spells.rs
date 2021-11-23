use docx::{document::BodyContent, DocxFile};
use itertools;
use regex::Regex;
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    pub text: String,
    pub repeated: bool,
    pub word_position: u32,
    pub paragraph: u32,
}

impl Word {
    pub fn represent(&self) -> String {
        let word_buff = vec![' '; 20 - self.text.len()]
            .into_iter()
            .collect::<String>();

        let paragraph_buff = vec![' '; 10 - (self.paragraph + 1).to_string().len()]
            .into_iter()
            .collect::<String>();
        format!(
            "Word: {}{}Paragraph: {}{}Word Position: {}",
            self.text,
            word_buff,
            self.paragraph + 1,
            paragraph_buff,
            self.word_position + 1
        )
    }
}

pub fn split_text_into_words(s: String) -> Vec<Word> {
    let seperator = Regex::new(r"([ !',.\n]+)").expect("Invalid regex");

    let re = Regex::new(r"([\w']+)").unwrap();

    // get all our words
    let paragraphs_of_word_arrays: Vec<Vec<Word>> = s
        .lines()
        // i is used later to indicate paragraph that owns the word.
        .enumerate()
        .map(|(i, line)| {
            seperator
                .split(line)
                .into_iter()
                .filter_map(|word| {
                    let trimmed_word = &re.captures_iter(word).next();
                    match trimmed_word {
                        Some(trimmed) => Some(trimmed[0].to_string()),
                        None => None,
                    }
                })
                .enumerate()
                .map(|(j, text)| Word {
                    text,
                    repeated: false,
                    paragraph: i as u32,
                    word_position: j as u32,
                })
                .collect::<Vec<Word>>()
        })
        .collect();

    itertools::concat(paragraphs_of_word_arrays)
}

pub fn mark_up(v: Vec<Word>, stop_words: Vec<&str>, buffer_length: usize) -> Vec<Word> {
    v.clone()
        .into_iter()
        .enumerate()
        .map(|(i, word)| {
            let lowercase_word = word.text.to_lowercase();
            if stop_words.contains(&&lowercase_word.as_ref()) {
                return word;
            }

            let end = if i + buffer_length + 1 > v.len() {
                v.len()
            } else {
                i + buffer_length + 1
            };

            if v[i + 1..end]
                .into_iter()
                .any(|x| x.text.to_lowercase() == lowercase_word)
            {
                return Word {
                    text: word.text,
                    repeated: true,
                    paragraph: word.paragraph,
                    word_position: word.word_position,
                };
            }

            word
        })
        .collect::<Vec<Word>>()
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
