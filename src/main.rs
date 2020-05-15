/*
 * Prototype of a OCH downloader in rust.
 */

pub struct Account
{
    username: String,
    password: String,
}

pub fn authenticate(username: &String, password: &String)
{
    //TODO: authenticate user and save as a session
}

pub fn download(url: &String, path: &String, account: &Account)
{
    if !account.username.is_empty() && !account.password.is_empty()
    {
        authenticate(&account.username, &account.password);
    }
    //TODO: download file from url
}

fn main()
{
    let username = String::from("username");
    let password = String::from("password");
    let path = String::from("/path/to/download");
    let url = String::from("url");
    let acc = Account {username, password};
    download(&url, &path, &acc)
}
