use reqwest::{Client, Response};
use reqwest::header::{Cookie, SetCookie, CookieJar};
use regex::Regex;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScarletData {
    transfert_volume: f64,
    max_volume: f64,
    days_left: u32,
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

    pub fn days_left(&self) -> u32 {
        self.days_left
    }

    fn login(username: &str, password: &str) -> Cookie {
        let url = "https://www.scarlet.be/customercare/logon.do?language=fr";

        let mut params = HashMap::new();
        params.insert("username", username);
        params.insert("password", password);

        let client: Client = Client::new().expect("Couldn't create client");

        let response: Response = client.post(url)
            .form(&params)
            .send()
            .expect("Failed to send login request");

        let set_cookie: &SetCookie = response.headers().get::<SetCookie>().expect("No cookie returned on login");

        let mut cookie_jar = CookieJar::new(b"");

        set_cookie.apply_to_cookie_jar(&mut cookie_jar);

        Cookie::from_cookie_jar(&cookie_jar)
    }

    fn get_consomation(cookies: Cookie) -> (f64, f64, u32) {
        let url = "https://www.scarlet.be/customercare/usage/dispatch.do";

        let client: Client = Client::new().expect("Couldn't create client");

        let mut response: Response = client.get(url)
            .header(cookies)
            .send()
            .expect("Failed to send request")
        ;

        let mut buf = Vec::new();
        response.read_to_end(&mut buf).expect("Failed to read response");

        let body = String::from_iter(buf.into_iter().map(|c| c as char));

        let volume_regex: Regex = Regex::new(r#"Math.round\(([0-9.]+)\)"#).unwrap();
        let days_left_regex: Regex = Regex::new(r#"(\d+) jour"#).unwrap();

        let mut volume_captures = volume_regex.captures_iter(body.as_ref());

        let transfert_volume: f64 = volume_captures
            .next()
            .expect("Can't match volume_regex for transfert_volume")
            .at(1)
            .unwrap()
            .parse()
            .expect("Could not parce transfert volume to f64");

        let max_volume: f64 = volume_captures
            .next()
            .expect("Can't match volume_regex for max_volume")
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

        (transfert_volume, max_volume, days_left)
    }
}