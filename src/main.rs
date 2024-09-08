mod tests;

#[macro_use]
extern crate rocket;

use passwords::{analyzer, scorer, PasswordGenerator};
use rocket::serde::json::Json;

#[derive(serde::Serialize, serde::Deserialize)]
struct Pwd {
    password: String,
    score: u8,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PasswordOptions {
    count: Option<usize>,
    length: Option<usize>,
    numbers: Option<bool>,
    lowercase_letters: Option<bool>,
    uppercase_letters: Option<bool>,
    symbols: Option<bool>,
    spaces: Option<bool>,
    exclude_similar_characters: Option<bool>,
    strict: Option<bool>,
}

impl PasswordOptions {
    fn new(count: usize, length: usize) -> PasswordOptions {
        PasswordOptions {
            count: Some(count),
            // Default password length
            length: Some(length),
            numbers: Some(true),
            lowercase_letters: Some(true),
            uppercase_letters: Some(true),
            symbols: Some(false),
            spaces: Some(true),
            exclude_similar_characters: Some(false),
            strict: Some(false),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Eg eter blåbærsyltetøy!"
}

#[post("/", data = "<password_options>")]
fn post_pwd(password_options: Json<PasswordOptions>) -> Json<Vec<Pwd>> {
    generate_passwords(&password_options)
}

#[get("/pwd")]
fn pwd() -> Json<Vec<Pwd>> {
    pwd_count(5)
}

#[get("/pwd/<count>")]
fn pwd_count(count: usize) -> Json<Vec<Pwd>> {
    let c = match count {
        1..=31 => count,
        _ => 31,
    };

    let password_options = PasswordOptions::new(c, 20);

    generate_passwords(&password_options)
}

fn generate_passwords(password_options: &PasswordOptions) -> Json<Vec<Pwd>> {
    let c = password_options.count.unwrap_or_else(|| 5);
    let pwd_length = password_options.length.unwrap_or_else(|| 20);
    let mut option_numbers = password_options.numbers.unwrap_or_else(|| true);
    let mut option_lowercase_letters = password_options.lowercase_letters.unwrap_or_else(|| true);
    let mut option_uppercase_letters = password_options.uppercase_letters.unwrap_or_else(|| true);
    let mut option_symbols = password_options.symbols.unwrap_or_else(|| false);
    let mut option_spaces = password_options.spaces.unwrap_or_else(|| true);
    let option_exclude_similar_characters = password_options
        .exclude_similar_characters
        .unwrap_or_else(|| false);
    let option_strict = password_options.strict.unwrap_or_else(|| false);

    if option_strict {
        option_numbers = true;
        option_lowercase_letters = true;
        option_uppercase_letters = true;
        option_symbols = true;
        option_spaces = true;
    }

    let pg = PasswordGenerator {
        length: pwd_length,
        numbers: option_numbers,
        lowercase_letters: option_lowercase_letters,
        uppercase_letters: option_uppercase_letters,
        symbols: option_symbols,
        spaces: option_spaces,
        exclude_similar_characters: option_exclude_similar_characters,
        strict: option_strict,
    };

    let mut pwd: Vec<Pwd> = Vec::with_capacity(c);
    pg.generate(c)
        .unwrap()
        .into_iter()
        .map(|x| {
            pwd.push(Pwd {
                password: x.clone(),
                score: scorer::score(&analyzer::analyze(&x)).ceil() as u8,
            });
        })
        .count();
    Json(pwd)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![post_pwd])
        .mount("/", routes![pwd])
        .mount("/", routes![pwd_count])
}
