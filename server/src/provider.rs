use std::error::Error;
use std::str;

use std::process::{Command, Stdio};

use crate::database;

extern crate base64;

type BErr = Box<dyn Error>;
type BRes<T> = Result<T, BErr>;

// pub enum Vendors {
//     Carrefour,
//     Casino,
//     Franprix,
//     Intermarche,
//     Monoprix
// }

fn generate_user_agent() -> String {
    return "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0".to_string();
}

fn develop_ssc<'a>(raw: String) -> String {
    let mut out = String::new();

    let mut temp: &str;
    for part in raw.split(",") {
        let raw = base64::decode(part).unwrap();
        temp = str::from_utf8(&raw).unwrap();

        if temp.len() >= 0 {
            out += "--cookie ";
            out += temp;
            out += " ";
        }
    }

    return out
}


pub trait Item {
    fn get_price(&self) -> BRes<f64>;
    fn fetch_and_push_update(&self, updates: &mut Vec<database::Update>) -> BRes<()>;

    fn get_carrefour(&self) -> BRes<f64>;
}

impl Item for database::QueryPart {
    fn get_price(&self) -> BRes<f64> {
        return match self.internal_shop_id {
            1 => self.get_carrefour(),

            _ => panic!("Not implemented yet :(")
        }
    }

    fn fetch_and_push_update(&self, updates: &mut Vec<database::Update>) -> BRes<()> {
        let price = self.get_price()?;

        let update = database::Update::construct(self, price);

        updates.push(update);

        return Ok(())
    }

    fn get_carrefour(&self) -> BRes<f64> {
        let curl = Command::new("curl")
            .arg("https://www.carrefour.fr".to_owned() + &self.external_item_id)
            .arg(&develop_ssc(self.ssc.to_string()))
            .arg("-A")
            .arg(generate_user_agent())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();



        let grep = Command::new("pcregrep")
            .arg("-M")
            .arg("-o1")
            .arg("\"<span.+class=\\\"[a-zA-Z0-9\\-\\_ ]*product-card-price__price--final\\\"[a-zA-Z0-9\\-\\_ =\\\"]*>[\\n ]*([0-9\\,]*)€[\\n ]*<\\/span>\"")
            .stdin(Stdio::from(curl.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let output_raw = grep.wait_with_output().unwrap();
        println!("{}", str::from_utf8(&output_raw.stdout).unwrap());

        let output = str::from_utf8(&output_raw.stdout).unwrap().split("\n").collect::<Vec<&str>>()[0];

        let res = output.parse::<f64>()?;

        return Ok(res);
    }
}
