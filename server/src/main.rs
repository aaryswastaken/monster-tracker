use std::env;

use fern;
use fern::colors::Color;

use humantime;
use log::{error, info, trace, warn};

use tokio;

use std::time::Duration;
use std::thread;

use mysql::{Pool, PooledConn};

mod provider;
mod database;
mod errors;

use crate::provider::Item;

fn value_or_default<T, E>(result: Result<T, E>, default: T) -> T {
    return match result {
        Ok(v) => v,
        Err(_) => default
    }
}

fn request_wrapper(conn: &mut PooledConn, i: i64) {
    let log_target = "SC#".to_owned()+&i.to_string();

    trace!(target: &log_target, "Entering new thread");
    let queries = database::get_queries(conn).expect("Hmmmm no queries?????");

    info!(target: &log_target, "There is {} queries to issue", queries.len());

    let mut updates: Vec<database::Update> = Vec::new();
    let mut errors: u16 = 0;

    for query in queries {
        match query.fetch_and_push_update(&mut updates) {
            Ok(_) => continue,
            Err(_) => {
                warn!("Query {} has had a problem", query);
                errors += 1;
            }
        }
    }

    info!(target: &log_target, "Updates is {} long, there has been a total of {} errors while executing the queries", updates.len(), errors);

    let tot = database::launch_all(conn, updates).expect("hmmm no updates????");

    info!("Launched {} requests", tot);
}

fn setup_logging() -> Result<(), fern::InitError> {
    let base_config = fern::Dispatch::new();
    let color_config = fern::colors::ColoredLevelConfig::new()
        .trace(Color::Blue)
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    // Perform allocation-free log formatting
    base_config.format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] ({}) {} - {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.target(),
                color_config.color(record.level()),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Trace)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("scraper.log")?)
        // Apply globally
        .apply()?;

    return Ok(());
}

const DEFAULT_PERIOD: Duration = Duration::from_secs(15 * 60);

#[tokio::main]
async fn main() {
    setup_logging().expect("Tkt");

    info!("Starting scraper...");

    let username = env::var("DB_USERNAME").map_err(|_| error!("No username has been provided")).unwrap();
    let password = env::var("DB_PASSWORD").map_err(|_| error!("No username has been provided")).unwrap();


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


    info!("Trying to connect to the database using {}@{}", username, host);
    let pool: Pool = database::connect(host, username, password, port, schema)
            .map_err(|e| error!("Database connection failed with: {}", e)).unwrap();


    info!("Starting the loop with a period of {}", period.as_secs() / 60);

    let mut i = 0;

    loop {
        trace!("Getting a new conn from the pool");
        let mut conn: PooledConn = pool.get_conn().expect("There has been an error in the attempt to fetch a new connection");

        trace!("Spawning new thread, SC#{}", i);
        tokio::spawn(async move {
            request_wrapper(&mut conn, i);
        });

        i += 1;

        thread::sleep(period);
    }
}
