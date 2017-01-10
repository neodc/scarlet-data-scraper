extern crate reqwest;
extern crate regex;
extern crate rustc_serialize;
extern crate toml;

mod config;
mod scarlet_data;

use std::env;
use std::path::Path;

use scarlet_data::ScarletData;

fn main() {
    let arg: Option<String> = env::args().nth(1);

    let config = config::Config::load(arg.as_ref().map(|str| Path::new(str)));

    let scarlet_data = ScarletData::load(config.username(), config.password());

    println!("{:#?}", scarlet_data);
}
