extern crate reqwest;
extern crate regex;
extern crate rustc_serialize;
extern crate toml;
#[macro_use]
extern crate mysql;

mod config;
mod scarlet_data;
mod database;

use std::env;
use std::path::Path;

use database::Database;
use scarlet_data::ScarletData;

fn main() {
    let arg = env::args().nth(1);

    let config = config::Config::load(arg.as_ref().map(|str| Path::new(str)));

    println!("Loading data...");
    let scarlet_data = ScarletData::load(config.username(), config.password());

    println!("transfert_volume: {}", scarlet_data.transfert_volume());
    println!("days_left: {}", scarlet_data.days_left());

    println!("Saving...");
    Database::new(config.database_url()).add_scarlet_data(&scarlet_data);
}
