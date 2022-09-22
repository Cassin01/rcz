use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
extern crate confy;
extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Conf<'a> {
     #[serde(borrow)]
    types: Vec<Type<'a>>,
    emoji: bool,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Conf {
            types: vec![
                Type::new("fix", "A bug fix", "🐛"),
                Type::new("feat", "A new feature", "✨"),
                Type::new("BREAKING CHANGE", "Changes that introduces a breaking API change", "💥"),
                Type::new("chore", "build system or external dependencies", "🛠️"),
                Type::new("ci", "CI related changes", "💫"),
                Type::new("docs", "Documentation only changes", "✏️"),
                Type::new("style", "Changes that do not affect the meaning of the code", "💄"),
                Type::new("refactor", "A code change that neither fixes a bug nor adds a feature", "🧹"),
                Type::new("perf", " A code change that improves performance", "🚄"),
                Type::new("test", "Adding or correcting tests", "🧪"),
            ],
            emoji: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct Type<'a> {
    description: &'a str,
    value: &'a str,
    emoji: &'a str,
}

impl<'a> Type<'a> {
    fn new(value: &'a str, description: &'a str, emoji: &'a str) -> Self {
        Self {
            description,
            value,
            emoji,
        }
    }
}

fn main() {
    let types = &[
        Type::new("fix", "A bug fix", "🐛"),
        Type::new("feat", "A new feature", "✨"),
        Type::new("BREAKING CHANGE", "Changes that introduces a breaking API change", "💥"),
        Type::new("chore", "build system or external dependencies", "🛠️"),
        Type::new("ci", "CI related changes", "💫"),
        Type::new("docs", "Documentation only changes", "✏️"),
        Type::new("style", "Changes that do not affect the meaning of the code", "💄"),
        Type::new("refactor", "A code change that neither fixes a bug nor adds a feature", "🧹"),
        Type::new("perf", " A code change that improves performance", "🚄"),
        Type::new("test", "Adding or correcting tests", "🧪"),
    ];
    let selections: Vec<String> = types
        .into_iter()
        .map(|x| format!("{}: {}", x.value.clone(), x.description.clone()))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{}", types[selection].value))
        .interact_text()
        .unwrap();
    println!("{}: {}", types[selection].value, input);
}
