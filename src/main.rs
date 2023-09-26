use dialoguer::{FuzzySelect, Input, theme::ColorfulTheme};

fn main() {
    let items = vec!["MIT License", "Apache License", "GNU GPLv3"];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter name")
        .interact_text()
        .unwrap();

    let year: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter year")
        .interact_text()
        .unwrap();

    println!("You chose: {}", items[selection]);
    println!("Your name: {}", name);
    println!("Your year: {}", year);
}
