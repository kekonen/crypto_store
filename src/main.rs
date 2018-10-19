#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate rocket;

use rocket_contrib::{Json};

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    side: bool, // sell: 1 true
    price: u64,
    amount: u64,
    ts: u64,
    id: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct TradeList {
    trades: Vec<Trade>
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    amount: u64,
    price: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct OrdersList {
    bids: Vec<Order>,
    asks: Vec<Order>,
    ts: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct OrdersList {
    bids: Vec<Order>,
    asks: Vec<Order>,
    ts: u64
}


#[post("/trades/<exchange>/<ticker>", format = "application/json", data = "<trades>")]
fn trades(exchange: String, ticker: String, trades: Json<TradeList>) -> String {
    format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].description)
}

#[post("/orders/<exchange>/<ticker>", format = "application/json", data = "<orders>")]
fn trades(exchange: String, ticker: String, orders: Json<orderList>) -> String {
    format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].description)
}

#[post("/orders/<exchange>/<ticker>", format = "application/json", data = "<orders>")]
fn trades(exchange: String, ticker: String, orders: Json<orderList>) -> String {
    format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].description)
}

fn main() {
    rocket::ignite().mount("/", routes![trades, orders, ticker]).launch();
}