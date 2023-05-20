use std::error::Error;
use std::fs::write;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

use log::{info, error};

use mysql::*;
use mysql::prelude::Queryable;

use chrono::prelude::*;

// TODO: add chrono crate

pub fn connect(host: String, username: String, password: String, port: String, db_name: String) -> Result<Pool, Box<dyn Error>> {
    let connection_url = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, db_name);

    let pool = Pool::new(Opts::from_url(&connection_url)? )?;

    return Ok(pool)
}

pub struct Update {
    item_id: u16,
    shop_id: u16,

    price: f64,
    query_epoch: u64
}

pub struct QueryPart {
    pub shop_id: u16,
    pub internal_shop_id: u16,
    pub shop_name: String,
    pub item_id: u16,
    pub item_name: String,
    pub external_item_id: String,
    pub ssc: String
}

impl fmt::Display for QueryPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.item_name, self.shop_name)
    }
}

trait Prepare {
    fn prepare(&self) -> String;
}

impl Prepare for u64 {
    fn prepare(&self) -> String {
        return Utc.timestamp(*self as i64, 0).to_rfc3339_opts(SecondsFormat::Secs, true)
                    .replace("T", " ").replace("Z", "");
    }
}

impl Update {
    pub fn construct(item: &QueryPart, price: f64) -> Self {
        let now = SystemTime::now();
        let epoch = now.duration_since(UNIX_EPOCH).expect("time went backwards").as_secs();

        return Update { item_id: item.item_id, shop_id: item.shop_id, price, query_epoch: epoch };
    }

    fn launch(&self, conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
        conn.exec_drop("INSERT INTO prices ('item_id', 'shop_id', 'date', 'value') VALUES (?)",
                (self.item_id, self.shop_id, self.query_epoch.prepare(), self.price, )
            )?;

        return Ok(());
    }
}

pub fn launch_all(conn: &mut PooledConn, updates: Vec<Update>) -> Result<u16, Box<dyn Error>> {
    let len_updates = updates.len();

    if len_updates > 0 {
        let mut query = "INSERT INTO prices (item_id, shop_id, date, value) VALUES ".to_string();
        let mut i = 0;


        for update in updates {
            query += &format!("({}, {}, \"{}\", {})", update.item_id, update.shop_id, update.query_epoch.prepare(), update.price);

            if i < len_updates - 1 {
                query += ", ";
            }

            i += 1;
        }

        info!("Issuing {}", query);

        let res: Vec<i64> = conn.query(query).map_err(|e| error!("There has been an issue in the mysql insertion: {}", e)).unwrap();

        println!("{:.?}", res);
    }

    return Ok(len_updates as u16);
}

pub fn get_queries(conn: &mut PooledConn) -> Result<Vec<QueryPart>, Box<dyn Error>> {
    let query = format!("select s.sid as 'shop_id', ve.internal_id as 'internal_shop_id', s.name as 'shop_name', i.id as 'item_id', i.name as 'item_name', si.eid as 'external_item_id', s.specific_cookie as 'shop_specific_cookie' from shops s, items i, specific_id si, vendors ve where si.uid = i.id and si.vid = ve.vid and s.vendor = ve.vid;");

    // ssc is for shop specific cookie
    let output = conn.query_map(query, |(shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc)| {
            return QueryPart { shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc}
        }).map_err(|e| error!("There has been an issue in the request list query: {}", e)).unwrap();

    return Ok(output);
}
