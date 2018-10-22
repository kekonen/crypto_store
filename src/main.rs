#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate time;


extern crate rocket;

use rocket::{State};
use rocket_contrib::{Json};
use std::collections::{VecDeque};
use std::collections::HashMap;
use std::sync::Mutex;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn get_time() -> String {
    let kek = time::now();
    return format!("{}-{}-{}-{}-{}",kek.tm_hour, kek.tm_min, kek.tm_mday, kek.tm_mon, kek.tm_year+1900);
}

fn write_file(path_way: &String, contents: &String) -> () {
    let path = Path::new(path_way);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(why) => {
            panic!("couldn't write ")
        },
        Ok(_) => println!("successfully wrote to"),
    }
}

struct TradesHandler {
    // exchanges: String,
    container: HashMap<String, Vec<Trade>>,
    path: String
}

type TradesHandlerMut = Mutex<TradesHandler>;


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

                container.insert(key, Vec::with_capacity(10000));
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

        if let Some(container) = self.container.get_mut(&key) {
            let capacity = container.capacity();
            let length = container.len();
            let trades_length = trades.len();
            println!("Container cap: {}, container length: {}, trades_length: {}", capacity, length, trades_length);
            if (length + trades_length) > capacity {
                let mut future_file = String::from("");
                for trade in container.clone() {
                    future_file.push('\n');
                    future_file.push_str(&serde_json::to_string(&trade).unwrap());
                }

                println!("full... writing file for key: {}", key);
                write_file(&format!("out/{}-{}.txt", key, get_time()), &future_file);
                
                container.clear();

                for &trade in trades {
                    // let trade_clone = trade.clone();
                    // println!("Pushing...");
                    container.push(trade.clone())
                }
                // let path = Path::new();


                // self.release(exchange, ticker).unwrap();
            } else {
                // make check batch
                let container_clone = container.clone();
                if length == 0 {
                    for &trade in trades {
                        // let trade_clone = trade.clone();
                        // println!("Pushing...");
                        container.push(trade.clone())
                    }
                } else {
                    let min_length = if length < 100 {length} else {100}; 
                    let check_batch = &container_clone[..min_length];
                    for &trade in trades {
                        // implement equal to trade, and compare latest ${trades_length or length or 0} on copies and not push if exists, pay attention on that the latest batch(see: 56 "checkbatch")
                        let trade_clone = trade.clone();
                        match check_batch.iter().find(|&&tr| tr == trade_clone) {
                            Some(_) => {
                                // println!("Found clone {}", &serde_json::to_string(&trade_clone).unwrap());
                                continue;
                            },
                            None => container.push(trade_clone),
                        }
                        // println!("Pushing...");
                        
                    }
                }
                
            }
            println!(" container.len {}, key {}", container.len(), key);
            
        }
    }

    // fn release(&mut self, exchange: String, ticker: String) -> Option<()> {
    //     //serialize vector of trades
    //     //push the file into the folder
    //     //clean the container
    //     Some(())
    // }

}



///

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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


struct Handlers {
    tradesHandler: TradesHandler
}

type HandlersMut = Mutex<Handlers>;


#[post("/trades/<exchange>/<ticker>", format = "application/json", data = "<trades>")]
fn trades(trades_handler: State<TradesHandlerMut>, exchange: String, ticker: String, trades: Json<TradeList>) -> String {
    let answer = format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, trades.trades[0].ts);
    // println!("{}", answer);
    let mut mut_handler = trades_handler.lock().unwrap();
    mut_handler.push(exchange, ticker, &trades.trades);
    answer
}

#[post("/orders/<exchange>/<ticker>", format = "application/json", data = "<orders>")]
fn orders(exchange: String, ticker: String, orders: Json<OrdersList>) -> String {
    let answer = format!("Incoming, exchange:{}, ticker {}, with data: {}!", exchange, ticker, orders.ts);
    // println!("{}", answer);
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
    .manage(Mutex::new(TradesHandler::new(exchanges, tickers, path)))
    .launch();
    
}

    // .manage(Mutex::new(Handlers { tradesHandler: TradesHandler::new(exchanges, tickers, path)}))
