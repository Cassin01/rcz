use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

fn main() {
    let selections = &[
        "fix:",
        "feat:",
        "BREAKING CHANGE:",
        "build:",
        "chore:",
        "ci:",
        "docs:",
        "style:",
        "refactor:",
        "perf:",
        "test:",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(selections[selection])
        .interact_text()
        .unwrap();
    println!("{} {}", selections[selection], input);
}
