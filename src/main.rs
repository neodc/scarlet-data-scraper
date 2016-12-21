extern crate reqwest;
extern crate regex;

use reqwest::Client;
use reqwest::header::{Cookie, CookiePair};
use regex::Regex;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let url = "https://www.scarlet.be/customercare/usage/dispatch.do";
    let cookie_name = "JSESSIONID".to_owned();
    let cookie_value = "6870C04D2F9757EA7B6F89F2DD3F3FC4".to_owned();

    let client: Client = Client::new().expect("Couldn't create client");

    let mut response = client.get(url)
        .header(Cookie(vec![CookiePair::new(cookie_name, cookie_value)]))
        .send()
        .expect("Failed to send request")
    ;

    let mut buf = Vec::new();
    response.read_to_end(&mut buf).expect("Failed to read response");

    let body = String::from_iter(buf.into_iter().map(|c| c as char));

    let transfert_volume_regex: Regex = Regex::new(r#"Math.round\(([0-9.]+)\)"#).unwrap();
    let days_left_regex: Regex = Regex::new(r#"(\d+) jour"#).unwrap();

    let transfert_volume: f64 = transfert_volume_regex
        .captures(body.as_ref())
        .expect("Can't match transfert_volume_regex")
        .at(1)
        .unwrap()
        .parse()
        .expect("Could not parce transfert volume to f64");

    let days_left: u32 = days_left_regex
        .captures(body.as_ref())
        .expect("Can't match days_left_regex")
        .at(1)
        .unwrap()
        .parse()
        .expect("Could not parce days left to u32");

    println!("{:?}", transfert_volume);
    println!("{:?}", days_left);
}
