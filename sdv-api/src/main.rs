#[macro_use] extern crate rocket;

use rand::seq::SliceRandom;
use rocket::serde::{Serialize, json::Json};

#[derive(Clone)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Joke {
  setup: String,
  punchline: String,
}

impl Joke {
    fn new(setup: &str, punchline: &str) -> Self {
        Self {
            setup: setup.to_string(),
            punchline: punchline.to_string(),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello Sup de Vinci Rennes!"
}

#[get("/")]
fn get_random_joke() -> Json<Joke> {
    let jokes = vec![
        Joke::new("Where does batman go to the bathroom?", "The batroom."),
        Joke::new("How do you make holy water?", "You boil the hell out of it"),
        Joke::new("What's red and bad for your teeth?", "A Brick."),
        Joke::new("How do you generate a random string?", "Put a Windows user in front of Vim and tell them to exit."),
        Joke::new("What do you call fake spaghetti?", "An impasta."),
        Joke::new("Why was the math book sad?", "Because it had too many problems."),
        Joke::new("How do you organize a space party?", "You planet."),
    ];
    Json(jokes.choose(&mut rand::thread_rng()).unwrap().clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/jokes/random", routes![get_random_joke])
}
