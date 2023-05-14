use std::env;

use tokio;

use std::time::Duration;
use std::thread;

mod provider;
mod database;

fn value_or_default<T, E>(result: Result<T, E>, default: T) -> T {
    return match result {
        Ok(v) => v,
        Err(_) => default
    }
}

fn request_wrapper() {
    panic!("todo :)");
}

const DEFAULT_PERIOD = Duration::from_secs(15 * 60);

#[tokio::main]
async fn main() {
    let username = env::var("DB_USERNAME").expect("No username has been provided");
    let password = env::var("DB_PASSWORD").expect("No password has been provided");

    let host = value_or_default(env::var("DB_SCHEMA"), 'monster_tracker');
    
    let port = value_or_default(env::var("DB_PORT"), '3306');
    let schema = value_or_default(env::var("DB_SCHEMA"), 'monster_tracker');

    let period = match env::var("SCRAPER_PERIOD") {
        Ok(v) => {
            return match v.parse::<i32>() {
                Ok(raw_period) => Duration::from_secs(raw_period * 60),
                Err(_) => DEFAULT_PERIOD
            }
        }
        Err(_) => DEFAULT_PERIOD
    };

    let mut (pool, conn) = database::connect(host, username, password, port, schema);

    loop {
        tokio::spawn(move {
            request_wrapper(conn);
        });

        thread::sleep(period);
    }
}
