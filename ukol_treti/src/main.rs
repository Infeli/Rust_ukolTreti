use slug::slugify;
use std::env;
use std::error;
use std::io;
extern crate csv;
use std::error::Error;
use std::fs::File;
use std::process;

fn vypis_po_znaku(input: &str) -> Result<(), &str> {
    Ok((for c in input.chars() {
        println!("{}", c);
    }))
}

fn vypis_delku(input: &str) -> Result<(), &str> {
    let delka = input.len();
    Ok((if delka == 1 {
        println!("Délka řetězce je {} znak", delka);
    } else {
        println!("Délka řetězce je {} znaků", delka);
    }))
}

fn vypis_pozpatku(input: &str) -> Result<(), &str> {
    let po_zpatku = input.chars().rev().collect::<String>();
    Ok((println!("Pozpátku: {}", po_zpatku)))
}

fn vypis_menu() {
    println!(
        "
    Vyber úpravu ... (vyber z uvozovek)
    ------------------------------------------
    'lowercase' = převede text na lowercase,
    'uppercase' = převede text na uppercase,
    'no-spaces' = odebere mezery, 
    'slugify' = konvertuje text do slug,
    'znaky' = vypíše text do znaků,
    'delka' = vypíše délku řetězce,
    'zpetne' = vypíše pozpátku
    "
    );
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
            String::from("[Error reading input]")
        }
    }
}

fn process_args(args: &mut Vec<String>, input: &str) {
    let slug = slugify(input);
    args.push(slug.trim().to_string());
}

fn handle_user_choice(
    choice: &str,
    args: &Vec<String>,
    slug: &str,
) -> Result<String, &'static str> {
    match choice {
        "lowercase" => println!("Do lowercase: {}", args[1].to_lowercase()),
        "uppercase" => println!("Do uppercase: {}", args[1].to_uppercase()),
        "no-spaces" => println!("Bez mezer: {}", args[1].replace(" ", "")),
        "slugify" => println!("Do slug: {}", slugify(slug)),
        "znaky" => vypis_po_znaku(&args[1]).expect("Nepodařilo se vypsat po znaku"),
        "delka" => vypis_delku(&args[1]).expect("Nepodařilo se vypsat délku"),
        "zpetne" => vypis_pozpatku(&args[1]).expect("Nepodařilo se vypsat pozpatku"),
        _ => println!("Neplatná volba!"),
    }
}

/*
Upřímně ... moc se v tom nevyznám. Ty errory mi dělají problém a nevím jak to mám implementovat.
Takže jsem provedl jen nějakou restrukturalizaci. Na tom zbytku jsem se zasekl ... Ještě se na to
podívám, ale oproti Jave nebo C#, tak v tom mám celkem guláš.
*/

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input = read_input("Zadej text:");

    process_args(&mut args, &input);

    vypis_menu();

    let volba = read_input("Vyber možnost:");
    handle_user_choice(&volba, &args, &args[1]);
}
