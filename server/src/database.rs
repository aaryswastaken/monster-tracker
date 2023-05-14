use std::error::Error;
use std::result::Result;

use mysql::*;
use mysql::prelude::Queryable;

extern crate chrono;

use chrono::prelude::*;

// TODO: add chrono crate

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

trait Prepare {
    fn prepare(&self) -> String 
}

impl Prepare for u64 {
    fn prepare(&self) -> String {
        panic!("To implemen!")
    }
}

impl Update {
    fn launch(&self, conn: &mut Conn) -> Result<(), Box<dyn Error>> {
        conn.exec_drop("INSERT INTO prices ('item_id', 'shop_id', 'date', 'value') VALUES (?)",
                (self.item_id, self.shop_id, self.query_epoch.prepare(), self.price, )
            );

        return Ok(());
    }
}

pub fn launch_all(conn: &mut Conn, updates: Vec<Update>) -> Result<u16, Box<dyn Error>> {
    conn.exec_batch("INSERT INTO prices ('item_id', 'shop_id', 'date', 'value') VALUES (:item_id, :shop_id, :date, :value)",
            updates.iter.map(|p| params! {
                "item_id" => p.item_id,
                "shop_id" => p.shop_id,
                "date" => p.query_epoch.prepare(), 
                "value" => self.price
            })
        );

    return Ok(updates.len() as u16);
}

pub fn get_queries(conn: &mut Conn) -> Result<Vec<QueryPart>, Box<dyn Error>> {
    let query = format!("select s.sid as 'shop_id', ve.internal_id as 'internal_shop_id', s.name as 'shop_name', i.id as 'item_id', i.name as 'item_name', si.eid as 'external_item_id', s.specific_cookie as 'shop_specific_cookie' from shops s, items i, specific_id si, vendors ve where si.uid = i.id and si.vid = ve.vid and s.vendor = ve.vid;");

    // ssc is for shop specific cookie
    let output = conn.query_map(query, |(shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc)| {
            return QueryPart { shop_id, internal_shop_id, shop_name, item_id, item_name, external_item_id, ssc}
        })?;

    return Ok(output);
}
