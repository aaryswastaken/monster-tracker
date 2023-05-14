use std::error::Error;
use std::str;

use std::process::{Command, Stdio};
use regex::Regex;

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

fn wrap_ssc(command: &mut Command, raw: String) -> &mut Command {
    let mut temp: &str;

    for part in raw.split(",") {
        let raw = base64::decode(part).unwrap();
        temp = str::from_utf8(&raw).unwrap();

        command.arg("--cookie");
        command.arg(temp);
    }

    return command
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
        let curl = wrap_ssc(Command::new("curl")
            .arg("https://www.carrefour.fr/p".to_owned() + &self.external_item_id),
                self.ssc.to_string())
            .arg("-A")
            .arg(generate_user_agent())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let midway_raw = curl.wait_with_output().unwrap();
        let raw = str::from_utf8(&midway_raw.stdout).unwrap();
        println!("midway trash {}", raw);

        let re = Regex::new(r#"<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*>[\n ]*([0-9\,]*)€[\n ]*<\/span>"#).unwrap();

        let re_res = re.captures(raw).unwrap();
        let output = match re_res.get(1) {
            Some(e) => e.as_str(),
            None => ""
        }.replace(",", ".");

        println!("Output: {}", output);

        let res = output.parse::<f64>()?;

        return Ok(res);
    }
}
