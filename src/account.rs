pub struct Account<'a>
{
    username: &'a str,
    password: &'a str,
    pub premium: bool,
}

impl Account<'_> {
    pub fn new<'a>(username: &'a str, password: &'a str) -> Account<'a>
    {
        let mut acc = Account {
            username,
            password,
            premium: false
        };
        acc.get_premium_status();
        acc
    }

    pub fn get_premium_status(&mut self)
    {
        let prem = false;
        self.premium = prem;
    }

    pub fn authenticate(&self) -> &str
    {
        // TODO: implement some kind of API controller to support multiple hoster
        // functions for authentication and premium status
        let auth_url = &format!("https://uploaded.net/io/login?id={}&pw={}", self.username, self.password);

        /*TODO: authenticate user and save as a session
         *
         * params username, password (maybe token?)
         */
        let mut easy = curl::easy::Easy::new();
        easy.url(auth_url).unwrap();
        easy.post(true).unwrap();
        return "SESSION"
    }
}
