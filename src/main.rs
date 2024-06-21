#![feature(proc_macro_hygiene)]

mod fetch_price;
mod lamport;
mod script;
fn main() {
    fetch_price::fetch_price();
}
