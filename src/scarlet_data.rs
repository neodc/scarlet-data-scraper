use reqwest::{Client, Response};
use reqwest::header::{Cookie, SetCookie, CookieJar};
use regex::Regex;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike, Utc};

#[derive(Debug)]
pub struct ScarletData {
	transfert_volume: f64,
	max_volume: f64,
	days_left: Option<u32>,
}

impl ScarletData {
	pub fn load(username: &str, password: &str) -> Self {
		let cookies = Self::login(username, password);

		let (transfert_volume, max_volume, days_left) = Self::get_consomation(cookies);

		ScarletData {
			transfert_volume: transfert_volume,
			max_volume: max_volume,
			days_left: days_left,
		}
	}

	pub fn transfert_volume(&self) -> f64 {
		self.transfert_volume
	}

	pub fn max_volume(&self) -> f64 {
		self.max_volume
	}

	pub fn days_left(&self) -> Option<u32> {
		self.days_left
	}

	fn login(username: &str, password: &str) -> Cookie {
		let url = "https://www.scarlet.be/customercare/logon.do?language=fr";

		let mut params = HashMap::new();
		params.insert("username", username);
		params.insert("password", password);

		let client: Client = Client::new().expect("Couldn't create client");

		let response: Response = client.post(url).form(&params).send().expect(
			"Failed to send login request",
		);

		let set_cookie: &SetCookie = response.headers().get::<SetCookie>().expect(
			"No cookie returned on login",
		);

		let mut cookie_jar = CookieJar::new(b"");

		set_cookie.apply_to_cookie_jar(&mut cookie_jar);

		Cookie::from_cookie_jar(&cookie_jar)
	}

	fn get_consomation(cookies: Cookie) -> (f64, f64, Option<u32>) {
		let url_html = "https://www.scarlet.be/customercare/usage/detail.do?selectedCollId=0";
		let url_csv = "https://www.scarlet.be/customercare/usage/csvexport.do?selectedInvoiceFeature=0";

		let html = Self::get_page(url_html, cookies.clone());
		let csv = Self::get_page(url_csv, cookies);

		let volume_regex: Regex = Regex::new(r#"<th class="digit">([0-9,]+)  GB</th>"#).unwrap();
		let max_volume_regex: Regex = Regex::new("Votre volume de téléchargement est <b>([0-9]+)  GB</b>.").unwrap();
		let first_date_regex: Regex = Regex::new(r#"[0-9]{2}/[0-9]{2}/[0-9]{4}"#).unwrap();

		let max_volume: f64 = max_volume_regex.captures(&html).expect(
			"Can't match max_volume_regex for max_volume",
		)[1]
			.parse()
			.expect("Could not parse max_volume to f64");

		let transfert_volume: f64 = volume_regex.captures_iter(&html).last().expect(
			"Can't match volume_regex for transfert_volume",
		)[1]
			.replace(",", ".")
			.parse()
			.expect("Could not parse transfert volume to f64");

		let first_date_str = &first_date_regex.captures(&csv).expect(
			"Can't match first_date_regex",
		)[0];

		println!("{:?}", first_date_str);

		let first_date = NaiveDate::parse_from_str(first_date_str, "%d/%m/%Y").expect("Could not parse first_day as date");

		let next_first_date = if first_date.month() == 12 {
			NaiveDate::from_ymd(first_date.year() + 1, 1, first_date.day())
		} else {
			NaiveDate::from_ymd(first_date.year(), first_date.month() + 1, first_date.day())
		};

		let last_date = next_first_date.pred();

		let days_left = last_date
			.signed_duration_since(Utc::now().naive_utc().date())
			.num_days();

		(transfert_volume, max_volume, Some(days_left as u32))
	}

	fn get_page(url: &str, cookies: Cookie) -> String {
		let client: Client = Client::new().expect("Couldn't create client");

		let mut response: Response = client.get(url).header(cookies).send().expect(
			"Failed to send request",
		);

		let mut buf = Vec::new();
		response.read_to_end(&mut buf).expect(
			"Failed to read response",
		);

		String::from_iter(buf.into_iter().map(|c| c as char))
	}
}
