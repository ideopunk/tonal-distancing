extern crate library;
use anyhow::Result;
use library::{definitions, functions};
use std::path::PathBuf;

#[test]
fn raw_report_on_a_file() -> Result<(), definitions::TonalDistanceError> {
    let content = functions::get_content_from_file(PathBuf::from("../test_files/test3.txt"))?;

    let stop_words = functions::get_stop_words_from_string(Some(vec![]));

    let res = functions::tell_you_how_bad(content, 50, stop_words, definitions::ResponseType::Raw)?;

    match res {
        definitions::Response::VecOfRuns(resp) => {
            pretty_assertions::assert_eq!(
                resp,
                vec![
                    definitions::Run {
                        text: String::from("yes and "),
                        repeated: true
                    },
                    definitions::Run {
                        text: String::from(
                            "drew him down to me so he could feel my breasts all perfume "
                        ),
                        repeated: false
                    },
                    definitions::Run {
                        text: String::from("yes and "),
                        repeated: true
                    },
                    definitions::Run {
                        text: String::from("his heart was going like mad "),
                        repeated: false
                    },
                    definitions::Run {
                        text: String::from("and yes I "),
                        repeated: true
                    },
                    definitions::Run {
                        text: String::from("said "),
                        repeated: false
                    },
                    definitions::Run {
                        text: String::from("yes I "),
                        repeated: true
                    },
                    definitions::Run {
                        text: String::from("will "),
                        repeated: false
                    },
                    definitions::Run {
                        text: String::from("Yes."),
                        repeated: true
                    },
                ]
            );
        }
        _ => panic!(),
    }

    Ok(())
}

#[test]
fn formatted_report_on_a_file() -> Result<(), definitions::TonalDistanceError> {
    let content = functions::get_content_from_file(PathBuf::from("../test_files/test3.txt"))?;

    let stop_words = functions::get_stop_words_from_string(Some(vec![]));

    let res = functions::tell_you_how_bad(
        content,
        50,
        stop_words,
        definitions::ResponseType::Formatted,
    )?;

    match res {
        definitions::Response::Str(resp) => {
            pretty_assertions::assert_eq!(
                resp,
                "Word: yes                  Paragraph: 1                   Word Position: 1\nWord: and                  Paragraph: 1                   Word Position: 2\nWord: yes                  Paragraph: 1                   Word Position: 16\nWord: and                  Paragraph: 1                   Word Position: 17\nWord: and                  Paragraph: 1                   Word Position: 24\nWord: yes                  Paragraph: 1                   Word Position: 25\nWord: I                    Paragraph: 1                   Word Position: 26\nWord: yes                  Paragraph: 1                   Word Position: 28\nWord: I                    Paragraph: 1                   Word Position: 29\nWord: Yes.                 Paragraph: 1                   Word Position: 31");
        }
        _ => panic!(),
    }

    Ok(())
}
