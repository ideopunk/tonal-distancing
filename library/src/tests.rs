#[cfg(test)]
mod tests {
    use crate::definitions::*;
    use crate::functions::*;
    use anyhow::Result;
    use pretty_assertions;
    use std::path::PathBuf;

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
    fn test_parse_doc() -> Result<(), TonalDistanceError> {
        let docstr = parse_doc(PathBuf::from("../test_files/test.docx"))?;
        pretty_assertions::assert_eq!(docstr, "here\nI'm here-\nthe snow falling");
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
