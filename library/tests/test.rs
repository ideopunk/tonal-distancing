extern crate library;
use std::path::PathBuf;

#[test]
fn report_on_a_file() -> Result<(), library::TonalDistanceError> {
    let content = library::get_content_from_file(PathBuf::from("test_files/test3.txt"))?;

    let stop_words = library::get_stop_words_from_string(None);

    let res = library::tell_you_how_bad(content, 50, stop_words, library::ResponseType::Raw)?;

    match res {
        library::Response::VecOfRuns(resp) => {
            pretty_assertions::assert_eq!(
                resp,
                vec![
                    library::Run {
                        text: String::from("yes and "),
                        repeated: true
                    },
                    library::Run {
                        text: String::from(
                            "drew him down to me so he could feel my breasts all perfume "
                        ),
                        repeated: false
                    },
                    library::Run {
                        text: String::from("yes and "),
                        repeated: true
                    },
                    library::Run {
                        text: String::from("his heart was going like mad "),
                        repeated: false
                    },
                    library::Run {
                        text: String::from("and yes I "),
                        repeated: true
                    },
                    library::Run {
                        text: String::from("said "),
                        repeated: false
                    },
                    library::Run {
                        text: String::from("yes I "),
                        repeated: true
                    },
                    library::Run {
                        text: String::from("will "),
                        repeated: false
                    },
                    library::Run {
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
