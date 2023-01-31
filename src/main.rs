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

fn command(cmd: &str) -> Res<String> {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    };
    let ret = String::from_utf8(output.stdout)?;
    Ok(ret)
}

fn shell_command(name: &str) -> bool {
    let cs: Vec<char> = name.chars().collect();
    cs.first() == Some(&'{') && cs.last() == Some(&'}')
}

fn inline(name: &str, index: usize, types: &[Type]) -> String {
    match name {
        "emoji" => types[index].emoji.clone(),
        "type" => types[index].value.clone(),
        "description" => types[index].description.clone(),
        "scope" => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("scope")
                .interact_text()
                .unwrap();
            if input == " " {
                "".to_string()
            } else {
                "(".to_owned() + &input + ")"
            }
        }
        cmd if shell_command(cmd) => command(&cmd[1..cmd.len() - 1]).unwrap(),
        x => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(x)
                .interact_text()
                .unwrap();
            if input == " " { "".to_string() } else { input }
        }
    }
}

fn p_syn(line: &str, index: usize, types: &[Type]) -> Result<String, String> {
    let mut cs = line.chars();
    let mut ret = String::new();
    loop {
        if let Some(c) = cs.next() {
            if c == '{' {
                let mut queue = vec![];
                let mut r = String::new();
                loop {
                    if let Some(c) = cs.next() {
                        if c == '}' {
                            if queue.pop().is_some() {
                                r.push(c);
                            } else {
                                ret.push_str(&inline(&r, index, types));
                                break;
                            }
                        } else if c == '{' {
                            queue.push(c);
                            r.push(c);
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

fn parser(format: String, index: usize, types: &[Type]) -> String {
    let v: Vec<_> = format.split('\n').collect();
    v.iter().fold(String::new(), |mut sum, x| {
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
        .iter()
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

#[test]
fn test_parser() {
    let format = "{type}: {emoji}{{echo 'hehe'}}".to_string();
    let types = vec![Type::new("fix", "A bug fix", "🐛")];
    let ret = parser(format, 0, &types);
    assert_eq!(ret, "\nfix: 🐛hehe\n");
}

#[test]
fn test_parser2() {
    let format = "{type}: {emoji}\n{{echo 'hehe'}}".to_string();
    let types = vec![Type::new("fix", "A bug fix", "🐛")];
    let ret = parser(format, 0, &types);
    assert_eq!(ret, "\nfix: 🐛\nhehe\n");
}
