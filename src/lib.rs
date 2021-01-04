use reqwest::{Client, Result, Response};
use std::collections::HashMap;
use std::path::Path;

pub struct Account<'a> {
    username: &'a str,
    password: &'a str,
    pub premium: bool,
}

impl Account<'_> {
    pub fn new<'a>(username: &'a str, password: &'a str) -> Account<'a> {
        let mut acc = Account {
            username,
            password,
            premium: false,
        };
        acc.get_premium_status();
        acc
    }

    pub fn get_premium_status(&mut self) {
        let prem = false;
        self.premium = prem;
    }

    pub async fn authenticate(&self) -> Result<Response> {
        // TODO: make a post request to login and save the session cookie

        /* Host: ddownload.com
         * Params login=<username>&password=<HTMLEncodedPassword>
         */
        let mut params = HashMap::new();
        params.insert("login", self.username);
        params.insert("password", self.password);
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let resp = client
            .post("https://ddownload.com/login.html")
            .form(&params)
            .send()
            .await?;
        println!("{:#?}", resp);
        for cookie in resp.cookies() {
            println!(
                "COOKIE JAR: name = {}, value = {}",
                cookie.name(),
                cookie.value()
            );
        }
        Ok(resp)
    }
}

#[allow(dead_code)]
pub struct Package<'a> {
    name: String,
    parts: Vec<&'a Part<'a>>,
    progress: f64,
    size: f64,
}

#[allow(dead_code)]
impl Package<'_> {
    fn new<'a>(name: String, parts: Vec<&'a Part>, progress: f64, size: f64) -> Package<'a> {
        Package {
            name,
            parts,
            progress,
            size,
        }
    }
}

#[allow(dead_code)]
pub struct Part<'a> {
    hoster: String,
    name: String,
    path: &'a Path,
    progress: f64,
    size: f64,
    state: State,
}

#[allow(dead_code)]
impl Part<'_> {
    fn new<'a>(hoster: String, name: String, path: &'a Path, progress: f64, size: f64, state: State) -> Part<'a> {
        Part {
            hoster,
            name,
            path,
            progress,
            size,
            state,
        }
    }
}


#[allow(dead_code)]
enum State {
    Online,
    Offline,
}
