use std::error::Error;
use std::result::Result;

use mysql::*;
use mysql::prelude::Queryable;


pub struct Update {
    item_id: u16,
    shop_id: u16,

    price: f64,
    query_epoch: u64
}

pub struct QueryPart {
    shop_id: u16,
    internal_shop_id: u16,
    shop_name: String,
    item_id: u16,
    item_name: String,
    external_item_id: String,
    ssc: String
}

impl Update {
    fn launch(&self, conn: &mut Conn) -> Result<(), Box<dyn Error>> {
        panic!("Deprecated");

        return Ok(());
    }
}

pub fn launch_all(conn: &mut Conn, updates: Vec<Update>) -> Result<u16, Box<dyn Error>> {
    return Ok(0);
}

pub fn get_queries(conn: &mut Conn) -> Result<Vec<QueryPart>, Box<dyn Error>> {
    let query = format!("select s.sid as 'shop_id', ve.internal_id as 'internal_shop_id', s.name as 'shop_name', i.id as 'item_id', i.name as 'item_name', si.eid as 'external_item_id', s.specific_cookie as 'shop_specific_cookie' from shops s, items i, specific_id si, vendors ve where si.uid = i.id and si.vid = ve.vid and s.vendor = ve.vid;");

    // ssc is for shop specific cookie
    let output = conn.query_map(query, |(shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc)| {
            return QueryPart { shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc}
        })?;

    return Ok(output);
}
