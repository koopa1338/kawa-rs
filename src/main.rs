/*
 * Prototype of a OCH downloader in rust.
 *
 */

mod account;

//use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;
use account::Account;

}

}

pub fn download(_url: &str, _path: &Path, account: &Account)
{
    if account.premium {
        let _session = account.authenticate();
    } else {
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

    let acc = Account::new(username, password);
    download(&url, &path, &acc);
    println!("DONE");
}
