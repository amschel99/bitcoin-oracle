extern crate serde;
use serde::{Deserialize, Serialize};

extern crate reqwest;
use reqwest::Client;

extern crate clap;
use clap::Parser;

use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbaseTime {
    pub data: CoinTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinTime {
    pub iso: String,
    pub epoch: i64,
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version = "0.1.0",
    about = "BTC oracle - Command Line Interface (CLI) Application"
)]
struct Cli {
    #[clap(short, long, default_value = "30000")]
    interval: u64,

    #[clap(short, long, default_value = "10")]
    frequency: i32,
    #[clap(short, long, default_value = "USD")]
    rates: String,
}

pub fn fetch_price() {
    let args = Cli::parse();
    let mut count = 0i32;

    loop {
        if count == args.frequency {
            break;
        }

        let currency = "BTC";
        let rates = &args.rates;

        let quote_time = get_coin_time();
        let spot_price =
            get_coin_price("spot".to_string(), currency.to_string(), rates.to_string());
        let buy_price = get_coin_price("buy".to_string(), currency.to_string(), rates.to_string());
        let sell_price =
            get_coin_price("sell".to_string(), currency.to_string(), rates.to_string());

        let buy_price = buy_price.as_ref();
        let sell_price = sell_price.as_ref();

        let spread_price: f32 = (buy_price.unwrap().parse::<f32>().unwrap())
            - (&sell_price.unwrap().parse::<f32>().unwrap());

        println!(
            "{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}",
            quote_time.unwrap(),
            currency,
            rates,
            spot_price.unwrap(),
            buy_price.unwrap(),
            sell_price.unwrap(),
            spread_price.to_string()
        );

        sleep(Duration::from_millis(args.interval));
        count += 1;
    }
}

#[tokio::main]
async fn get_coin_price(
    request_type: String,
    request_currency: String,
    request_rates: String,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.coinbase.com/v2/prices/{currency}-{rates}/{type}",
        currency = request_currency,
        rates = request_rates,
        type = request_type
    );

    let client = Client::new();
    let resp_price = client
        .get(&request_url)
        .send()
        .await?
        .json::<CoinbasePrice>()
        .await?;

    Ok(resp_price.data.amount)
}

#[tokio::main]
async fn get_coin_time() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let request_url = format!("https://api.coinbase.com/v2/time");

    let client = Client::new();
    let resp_time = client
        .get(&request_url)
        .send()
        .await?
        .json::<CoinbaseTime>()
        .await?;

    Ok(resp_time.data.iso)
}
