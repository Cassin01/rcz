use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

#[allow(dead_code)]
struct Type<'a> {
    description: &'a str,
    value: &'a str,
    emoji: Option<&'a str>,
}

impl<'a> Type<'a> {
    fn new(value: &'a str, description: &'a str) -> Self {
        Self {
            description,
            value,
            emoji: None,
        }
    }
}

fn main() {
    let types = &[
        Type::new("fix", "A bug fix"),
        Type::new("feat", "A new feature"),
        Type::new("BREAKING CHANGE", "Changes that introduces a breaking API change"),
        Type::new("build", "Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)"),
        Type::new("chore", "Changes that updating grunt tasks etc; no production code change"),
        Type::new("ci", "Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)"),
        Type::new("docs", "Documentation only changes"),
        Type::new("style", "Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)"),
        Type::new("refactor", "A code change that neither fixes a bug nor adds a feature"),
        Type::new("perf", " A code change that improves performance"),
        Type::new("test", "Adding missing tests or correcting existing tests"),
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
        .with_prompt(format!("{}:", types[selection].value))
        .interact_text()
        .unwrap();
    println!("{} {}", selections[selection], input);
}
