/*
 * Prototype of a OCH downloader in rust.
 *
 */

//use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

pub struct Account<'a>
{
    username: &'a str,
    password: &'a str,
    premium: bool,
}

pub fn authenticate(username: &str, password: &str)
{
    // TODO: implement some kind of API controller to support multiple hoster
    // functions for authentication and premium status
    let auth_url = &format!("https://uploaded.net/io/login?id={}&pw={}", username, password);
    
    /*TODO: authenticate user and save as a session
     *
     * params username, password (maybe token?)
     */
    let mut easy = curl::easy::Easy::new();
    easy.url(auth_url).unwrap();
    easy.post(true).unwrap();
}

pub fn download(_url: &str, _path: &Path, account: &Account)
{
    if !account.username.is_empty() && !account.password.is_empty() || !account.premium
    {
        authenticate(&account.username, &account.password);
    }
    else
    {
        println!("No authentication, loading in free mode...");
    }
    // TODO: download file from url

    // FIXME: download data to buffer data

    /*
     * writes data to file
     *
     * let display = path.display();
     * let mut file = match File::create(&path) {
     *     Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
     *     Ok(file) => file,
     * };
     *
     * match file.write_all(data.as_bytes()) {
     *     Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
     *     Ok(_) => println!("successfully wrote to {}", display),
     * }
     */
}

fn main()
{
    // TODO: Read username, password and download path from a config file
    let username: &str = "username";
    let password: &str = "password";

    // TODO: use a database to store filename and url of added items
    let path = Path::new("./file");
    let url: &str = "url";

    let acc = Account {username, password, premium: false};
    download(&url, &path, &acc)
}
