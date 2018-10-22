#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate rocket;

use rocket::{State};
use rocket_contrib::{Json};
use std::collections::{VecDeque};
use std::collections::HashMap;

struct TradesHandler {
    // exchanges: String,
    container: HashMap<String, VecDeque<Trade>>,
    path: String
}

impl TradesHandler {
    fn new(exchanges: Vec<String>, tickers: Vec<String>, path: String) -> TradesHandler {
        let mut container = HashMap::new();
        for exchange in exchanges.iter() {
            let exch = exchange.clone();
            // let tickers_clone = tickers.clone();
            for ticker in tickers.iter() {
                let tick = ticker.clone();

                let key = format!("{}_{}", exch, tick);
                println!("Inserting key: {}", key);

                container.insert(key, VecDeque::with_capacity(10000));
                // match container.get(&ex) {
                //     Some(&e) => ex.insert(tick, VecDeque::new()),
                //     _ => None,
                // };
            }
        }

        TradesHandler{container, path}
    }

    fn push(&mut self, exchange: String, ticker: String, trades: &Vec<Trade>) -> () {
        let key = format!("{}_{}", exchange, ticker);
        // let container = self.container.get(&key);

        if let Some(container) = self.container.get_mut(&key) {
            let capacity = container.capacity();
            println!("cap: {}", capacity)
        }
    }

}



///

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
fn trades(handlers: State<Handlers>, exchange: String, ticker: String, trades: Json<TradeList>) -> String {
    let answer = format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].ts);
    println!("{}", answer);
    handlers.tradesHandler.push(exchange, ticker, &trades.trades);
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



struct Handlers {
    tradesHandler: TradesHandler
}


fn main() {
    // let mut translateExchange = HashMap::new();
    // translateExchange.insert("bitfinex".to_string(), Exchange::BITFINEX);
    // translateExchange.insert("kraken".to_string(), Exchange::KRAKEN);
    // translateExchange.insert("poloniex".to_string(), Exchange::POLOINEX);
    // translateExchange.insert("binance".to_string(), Exchange::BINANCE);
    // translateExchange.insert("bittrex".to_string(), Exchange::BITTREX);

    // let mut translateCurrency = HashMap::new();
    // translateCurrency.insert("BTCUSDT".to_string(), Currency::BTCUSDT);

    // let exchanges = Vec![Exchange::BITFINEX, Exchange::KRAKEN, Exchange::POLOINEX, Exchange::BINANCE, Exchange::BITTREX];
    // let currencies = Vec![Currency::BTCUSDT];

    // let router = Router {exchanges, currencies, translateExchange, translateCurrency};
    let exchanges = vec!["bitfinex".to_string(), "kraken".to_string(), "poloniex".to_string(), "binance".to_string(), "bittrex".to_string()];
    let tickers = vec!["BTCUSDT".to_string()];
    let path: String = ".".to_string();

    // let tradesHandler = TradesHandler::new(exchanges, tickers, path);

    // let handlers = Handlers {tradesHandler: &tradesHandler};
    
    rocket::ignite()
    .mount("/", routes![trades, orders, ticker])
    .manage(Handlers {tradesHandler: TradesHandler::new(exchanges, tickers, path)})
    .launch();
    
}