use crate::utils;

use crate::constants::Buffer;
use crate::constants::Snippet;
use crate::constants::SnippetKind;
use crate::constants::SnippetQuestion;

use crate::constants::RESET; 
use crate::constants::BOLD;

use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};

pub fn add_snippet( args: &Vec<String>, snippets: &mut Vec<Snippet>) {
    let to_add = match args.get(2) {
        Some(v) => v.parse().unwrap_or(1),
        None => 1,
    };

    for _ in 0..to_add {
        let mut questions: Vec<SnippetQuestion> = vec![
            SnippetQuestion {
                title: format!("{}Title{}   > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Content{} > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Tags{}    > ", BOLD, RESET),
                qtype: "input".to_string(),
                buf: Buffer::Text("".to_string()),
                answered: false,
            },
            SnippetQuestion {
                title: format!("{}Type{}    > ", BOLD, RESET),
                qtype: "enum".to_string(),
                buf: Buffer::Kind(SnippetKind::Note),
                answered: false,
            },
        ];

        while !utils::are_all_answered(&questions) {
            for q in questions.iter_mut().filter(|q| !q.answered) {
                print!("{}", q.title);
                io::stdout().flush().unwrap();

                let mut answ = String::new();
                io::stdin().read_line(&mut answ).unwrap();
                let answ = answ.trim();

                if q.qtype == "input" {
                    q.buf = Buffer::Text(answ.to_string());
                } else {
                    q.buf = match answ {
                        "command" => Buffer::Kind(SnippetKind::Command),
                        "json" => Buffer::Kind(SnippetKind::Json),
                        _ => Buffer::Kind(SnippetKind::Note),
                    };
                }
                q.answered = true;
            }
        }

        let last_id = utils::get_last_id(&snippets);
        snippets.push(Snippet {
            id: last_id + 1,
            title: utils::extract_text(&questions[0].buf),
            content: utils::extract_text(&questions[1].buf),
            tags: utils::extract_text(&questions[2].buf)
                .split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|tag| !tag.is_empty())
                .collect(),
            kind: utils::extract_kind(&questions[3].buf),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        });
        utils::print_success(&format!("Snippet #{} saved", last_id + 1));
    }
}