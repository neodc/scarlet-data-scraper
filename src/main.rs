extern crate reqwest;
extern crate regex;
extern crate rustc_serialize;
extern crate toml;
extern crate mysql;
extern crate teleborg;
extern crate chrono;

mod config;
mod scarlet_data;
mod database;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

use database::Database;
use scarlet_data::ScarletData;

fn main() {
	let arg = env::args().nth(1);

	let config = config::Config::load(arg.as_ref().map(|str| Path::new(str)));

	println!("Loading data...");
	let scarlet_data = ScarletData::load(config.username(), config.password());

	println!("transfert_volume: {}", scarlet_data.transfert_volume());
	println!("max_volume: {}", scarlet_data.max_volume());
	print!("days_left: ");
	match scarlet_data.days_left() {
		Some(days_left) => println!("{}", days_left),
		None => println!("NULL"),
	}

	println!("Saving...");
	let database = Database::new(config.database_url());

	database.add_scarlet_data(&scarlet_data);

	if let Some(days_left) = scarlet_data.days_left() {
		let volume_by_days_left = (scarlet_data.max_volume() - scarlet_data.transfert_volume()) / days_left as f64;
		println!("volume_by_days_left: {}", volume_by_days_left);

		let since_last_day = database.get_consomation_since_last_day(&scarlet_data);

		if since_last_day > volume_by_days_left {
			let message = format!("WARNING {:.2}Go consumed last day but only {:.2}Go left per day", since_last_day, volume_by_days_left);

			println!("{}", message);

			send_notification(&config, message.as_str());
		}
	}
}

fn send_notification(config: &config::Config, message: &str) {
	let filename = "last_message";

	if let Ok(mut file) = File::open(filename) {
		let mut contents = String::new();
		if let Ok(_) = file.read_to_string(&mut contents) {
			if let Ok(time) = chrono::DateTime::parse_from_rfc3339(contents.as_str()) {
				if (chrono::Utc::now().timestamp() - time.timestamp()) <= 60 * 60 * 24 {
					return;
				}
			}
		}
	}

	let token = config.telegram_token();

	let chat_id = -183853562;

	let bot = teleborg::Bot::new(format!("https://api.telegram.org/bot{}", token)).unwrap();

	if let Ok(_) = bot.send_message(&chat_id, message, None, None, None, None, None) {
		if let Ok(mut file) = File::create(filename) {
			let _ = file.write_all(chrono::Utc::now().to_rfc3339().as_bytes());
		}
	}

	/*
	Use to get the id of the chat

	let mut dispatcher = teleborg::Dispatcher::new();

	dispatcher.add_command_handler("test", |bot: &teleborg::Bot, update: teleborg::objects::Update, args: Option<Vec<&str>>| {
		bot.reply_to_message(&update, "It works!").unwrap();

		println!("{:#?}", update);
	}, false);

	teleborg::Updater::start(Some(token.to_string()), None, None, None, dispatcher);
	*/
}
