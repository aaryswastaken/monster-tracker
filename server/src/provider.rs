use std::error::Error;

use crate::database;

pub enum Vendors {
    Carrefour,
    Casino,
    Franprix,
    Intermarche,
    Monoprix
}

struct Item {
    item_id: u16,
    shop_id: u16,
    shop_type: Vendors
}

impl Item {
    fn get_price(&self) -> Result<f64, Box<dyn Error>> {
        return Ok(0 as f64)
    }

    fn fetch_and_push_update(&self, updates: &Vec<database::Update>) -> Result<(), Box<dyn Error>> {
        return Ok(())
    }
}
