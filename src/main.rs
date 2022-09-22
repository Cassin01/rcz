use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
extern crate confy;
extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Conf {
    format: String,
    types: Vec<Type>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            format: "{type}: {subject}".to_string(),
            types: vec![
                Type::new("fix", "A bug fix", "🐛"),
                Type::new("feat", "A new feature", "✨"),
                Type::new(
                    "BREAKING CHANGE",
                    "Changes that introduces a breaking API change",
                    "💥",
                ),
                Type::new("chore", "build system or external dependencies", "🛠️"),
                Type::new("ci", "CI related changes", "💫"),
                Type::new("docs", "Documentation only changes", "✏️"),
                Type::new(
                    "style",
                    "Changes that do not affect the meaning of the code",
                    "💄",
                ),
                Type::new(
                    "refactor",
                    "A code change that neither fixes a bug nor adds a feature",
                    "🧹",
                ),
                Type::new("perf", " A code change that improves performance", "🚄"),
                Type::new("test", "Adding or correcting tests", "🧪"),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Type {
    description: String,
    value: String,
    emoji: String,
}

impl Type {
    fn new(value: &str, description: &str, emoji: &str) -> Self {
        Self {
            description: description.to_string(),
            value: value.to_string(),
            emoji: emoji.to_string(),
        }
    }
}

fn inline(name: &str, index: usize, types: &Vec<Type>) -> String {
    match name {
        "emoji" => types[index].emoji.clone(),
        "type" => types[index].value.clone(),
        "subject" => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "subject"))
                .interact_text()
                .unwrap();
            input
        }
        "body" => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "body"))
                .interact_text()
                .unwrap();
            input
        }
        "footer" => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "footer"))
                .interact_text()
                .unwrap();
            input
        }
        "scope" => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{}", "scope"))
                .interact_text()
                .unwrap();
            let ret = "(".to_owned() + &input + ")";
            ret
        }
        _ => "".to_string(),
    }
}

fn p_syn(line: &str, index: usize, types: &Vec<Type>) -> Result<String, String> {
    let mut cs = line.chars();
    let mut ret = String::new();
    loop {
        if let Some(c) = cs.next() {
            if c == '{' {
                let mut r = String::new();
                loop {
                    if let Some(c) = cs.next() {
                        if c == '}' {
                            ret.push_str(&inline(&r, index, types));
                            break;
                        } else {
                            r.push(c);
                        }
                    } else {
                        return Err("There is no right bract".to_string());
                    }
                }
            } else {
                ret.push(c);
            }
        } else {
            return Ok(ret);
        }
    }
}

fn parser(format: String, index: usize, types: &Vec<Type>) -> String {
    let v: Vec<_> = format.split('\n').collect();
    v.into_iter().fold(String::new(), |mut sum, x| {
        sum.push('\n');
        sum.push_str(&p_syn(x, index, types).unwrap());
        sum
    })
}

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn core() -> Res<()> {
    let cfg: Conf = confy::load("rcz", None)?;
    let types = &cfg.types;
    let selections: Vec<String> = types
        .into_iter()
        .map(|x| format!("{}: {}", x.value.clone(), x.description.clone()))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()?;

    let format = parser(cfg.format, selection, &cfg.types);
    println!("{}", format);
    Ok(())
}

fn main() -> Res<()> {
    core()?;
    Ok(())
}
