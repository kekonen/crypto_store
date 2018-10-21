#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate rocket;

use rocket_contrib::{Json};

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
    side: bool, // sell: 1 true
    price: f64,
    amount: f64,
    ts: u64,
    id: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct TradeList {
    trades: Vec<Trade>
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderLevel {
    amount: f64,
    price: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct OrdersList {
    bids: Vec<OrderLevel>,
    asks: Vec<OrderLevel>,
    ts: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticker {
    symbol: String,
    high: f64,
    low: f64,
    bid: f64,
    ask: f64,
    open: f64,
    close: f64,
    last: f64,
    ts: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct TickerList {
    tickers: Vec<Ticker>
}


#[post("/trades/<exchange>/<ticker>", format = "application/json", data = "<trades>")]
fn trades(exchange: String, ticker: String, trades: Json<TradeList>) -> String {
    let answer = format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].ts);
    println!("{}", answer);
    answer
}

#[post("/orders/<exchange>/<ticker>", format = "application/json", data = "<orders>")]
fn orders(exchange: String, ticker: String, orders: Json<OrdersList>) -> String {
    let answer = format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, orders.ts);
    println!("{}", answer);
    answer
}

#[post("/ticker/<exchange>", format = "application/json", data = "<ticker>")]
fn ticker(exchange: String, ticker: Json<Ticker>) -> String {
    format!("Incoming, exchange:{}, with data: {}!", exchange, ticker.symbol)
}

#[post("/tickers/<exchange>", format = "application/json", data = "<tickers>")]
fn tickers(exchange: String, tickers: Json<TickerList>) -> String {
    format!("Incoming, exchange:{}, with data: {}!", exchange, tickers.tickers[0].symbol)
}

fn main() {
    rocket::ignite().mount("/", routes![trades, orders, ticker]).launch();
}