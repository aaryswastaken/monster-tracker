use std::error::Error;
use std::result::Result;

use mysql::*;

use crate::provider;

pub struct Update {
    item_id: u16,
    shop_id: u16,

    price: f64,
    query_epoch: u64
}

impl Update {
    fn launch(&self, conn: Conn) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}

pub fn launch_all(conn: Conn, updates: Vec<Update>) -> Result<u16, Box<dyn Error>> {
    return Ok(0);
}

pub fn get_queries(conn: Conn) -> Vec<provider::Item> {
    return vec![]
}
