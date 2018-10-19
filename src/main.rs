#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate rocket;

use rocket_contrib::{Json};

#[derive(Serialize, Deserialize, Debug)]
struct Trades {
    description: String,
    complete: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct TradeList {
    trades: Vec<Trades>
}


#[post("/trades/<exchange>/<ticker>", format = "application/json", data = "<trades>")]
fn trades(exchange: String, ticker: String, trades: Json<TradeList>) -> String {
    format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].description)
}

#[get("/orders/<name>/<age>")]
fn orders(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/ticker/<name>/<age>")]
fn ticker(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![trades, orders, ticker]).launch();
}