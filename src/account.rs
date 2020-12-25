extern crate reqwest;

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

    pub async fn authenticate(&self) -> reqwest::Result<reqwest::Response> {
        // TODO: make a post request to login and save the session cookie

        /* Host: ddownload.com
         * Params login=<username>&password=<HTMLEncodedPassword>
         */
        let mut params = std::collections::HashMap::new();
        params.insert("login", self.username);
        params.insert("password", self.password);
        let client = reqwest::Client::builder()
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
