use anyhow::{bail, Result};
use docx::DocxError;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub enum ResponseType {
    Raw,
    // Colorized,
    Formatted,
}

#[derive(Debug)]
pub enum Response {
    VecOfRuns(Vec<Run>),
    Str(String),
}

impl FromStr for ResponseType {
    type Err = anyhow::Error;

    fn from_str(res_type: &str) -> Result<Self, anyhow::Error> {
        match res_type {
            "raw" => Ok(ResponseType::Raw),
            // "colorized" => Ok(ResponseType::Colorized),
            "formatted" => Ok(ResponseType::Formatted),
            _ => bail!("Could not parse a response type"),
        }
    }
}

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
        let word_buff = vec![' '; (20 - self.pure_word.len() as i32).abs() as usize]
            .into_iter()
            .collect::<String>();

        let paragraph_buff = vec![' '; 20 - self.paragraph.to_string().len()]
            .into_iter()
            .collect::<String>();
        format!(
            "Word: {}{}Paragraph: {}{}Word Position: {}",
            self.original_word,
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

#[derive(Error, Debug)]
pub struct DocError(DocxError);

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self.0)
    }
}

///TonalDistanceError enumerates all possible errors returneed by this lib.
#[derive(Error, Debug)]
pub enum TonalDistanceError {
    /// Represents a regex error.
    #[error("Regex error")]
    RegexError { source: regex::Error },

    /// Represents a failure to read a docx file.
    #[error("Failed to read from docx file")]
    DocXReadError { source: DocError },

    /// Should not occur... ;)
    #[error("How did you get here?")]
    UhhhError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
