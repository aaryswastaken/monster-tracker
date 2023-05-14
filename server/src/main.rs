use std::env;

use tokio;

use std::time::Duration;
use std::thread;

use mysql::{Pool, PooledConn};

mod provider;
mod database;

use crate::provider::Item;

fn value_or_default<T, E>(result: Result<T, E>, default: T) -> T {
    return match result {
        Ok(v) => v,
        Err(_) => default
    }
}

fn request_wrapper(conn: &mut PooledConn) {
    let queries = database::get_queries(conn).expect("Hmmmm no queries?????");

    let mut updates: Vec<database::Update> = Vec::new();

    for query in queries {
        query.fetch_and_push_update(&mut updates).expect("Error while pew pew the request");
    }

    database::launch_all(conn, updates).expect("hmmm no updates????");
}

const DEFAULT_PERIOD: Duration = Duration::from_secs(15 * 60);

fn main() {
    let username = env::var("DB_USERNAME").expect("No username has been provided");
    let password = env::var("DB_PASSWORD").expect("No password has been provided");

    let host = value_or_default(env::var("DB_SCHEMA"), "127.0.0.1".to_string());

    let port = value_or_default(env::var("DB_PORT"), 3306.to_string());
    let schema = value_or_default(env::var("DB_SCHEMA"), "monster_tracker".to_string());

    let period = match env::var("SCRAPER_PERIOD") {
        Ok(v) => match v.parse::<u64>() {
                Ok(raw_period) => Duration::from_secs(raw_period * 60),
                Err(_) => DEFAULT_PERIOD
            }
        Err(_) => DEFAULT_PERIOD
    };

    let pool: Pool = database::connect(host, username, password, port, schema).unwrap();

    loop {
        let mut conn: PooledConn = pool.get_conn().expect("There has been an error in the attempt to fetch a new connection");
        tokio::spawn(async move {
            request_wrapper(&mut conn);
        });

        thread::sleep(period);
    }
}
