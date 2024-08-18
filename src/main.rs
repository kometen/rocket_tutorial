#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use passwords::PasswordGenerator;

struct Pwd {
    password: String,
    score: f64,
}

#[get("/")]
fn index() -> &'static str {
    "Eg eter blåbærsyltetøy!"
}

#[get("/pwd")]
fn pwd() -> Json<Vec<Pwd>> {
    pwd_count(5)
}

#[get("/pwd/<count>")]
fn pwd_count(count: usize) -> Json<Vec<Pwd>> {
    let c = match count {
        0..=31 => count,
        _ => 31
    };

    let pg = PasswordGenerator {
        length: 20,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    let mut pwd = Vec<Pwd>::new();
    Json(pg.generate(c).unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![pwd])
        .mount("/", routes![pwd_count])
}
