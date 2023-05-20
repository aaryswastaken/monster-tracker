use std::error::Error;
use std::str;

use log::{error, info, trace};

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
        let raw = base64::decode(part).map_err(|e| error!("There has been an issue in the base64 cookie parsing: {}", e)).unwrap();
        temp = str::from_utf8(&raw).map_err(|e| error!("There has been an issue in the base64 result accumulation: {}", e)).unwrap();

        command.arg("--cookie");
        command.arg(&temp);

        trace!("Using specific cookie {}", temp)
    }

    return command
}


pub trait Item {
    fn get_price(&self) -> BRes<f64>;
    fn fetch_and_push_update(&self, updates: &mut Vec<database::Update>) -> BRes<()>;

    fn get_carrefour(&self) -> BRes<f64>;
    fn get_intermarche(&self) -> BRes<f64>;
    fn get_casino(&self) -> BRes<f64>;
}

impl Item for database::QueryPart {
    fn get_price(&self) -> BRes<f64> {
        return match self.internal_shop_id {
            1 => self.get_carrefour(),
            2 => self.get_intermarche(),
            3 => self.get_casino(),

            _ => {
                error!("Tried to target an unsupported vendor. Id: {}", self.internal_shop_id);
                panic!("Not implemented yet :(");
            }
        }
    }

    fn fetch_and_push_update(&self, updates: &mut Vec<database::Update>) -> BRes<()> {
        let price = self.get_price().map_err(|e| error!("There has been an issue in the price fetching method: {}", e)).unwrap();

        let update = database::Update::construct(self, price);

        updates.push(update);

        return Ok(())
    }

    fn get_carrefour(&self) -> BRes<f64> {
        trace!("Issuing a request to carrefour.fr, using url:\n   carrefour.fr/p{}", &self.external_item_id);
        let curl = wrap_ssc(Command::new("curl")
            .arg("https://www.carrefour.fr/p".to_owned() + &self.external_item_id),
                self.ssc.to_string())
            .arg("-A")
            .arg(generate_user_agent())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| error!("There is an issue in the cURL: {}", e))
            .unwrap();

        let midway_raw = curl.wait_with_output()
                .map_err(|e| error!("There has been an error in the curl stdout fetch: {}", e)).unwrap();
        let raw = str::from_utf8(&midway_raw.stdout)
                .map_err(|e| error!("There has been an error in the curl out aggregation: {}", e)).unwrap();

        let re = Regex::new(r#"<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*>[\n ]*([0-9\,]*)€[\n ]*<\/span>"#)
                .map_err(|e| error!("There has been an error in the regex: {}", e)).unwrap();

        let re_res = match re.captures(raw) {
            Some(e) => e,
            None => {
                error!("There has been an issue in the capturing group");
                panic!("see log");
            }
        };

        let output = match re_res.get(1) {
            Some(e) => e.as_str(),
            None => ""
        }.replace(",", ".");

        println!("Output: {}", output);

        let res = output.parse::<f64>()
            .map_err(|e| error!("There has been an error in the result parsing: {}", e)).unwrap();

        return Ok(res);
    }

    fn get_intermarche(&self) -> BRes<f64> {
        trace!("Issuing a request to intermarche.com using url:\n    intermarche.com{}", &self.external_item_id);

        let curl = wrap_ssc(Command::new("curl")
            .arg("https://www.intermarche.com".to_owned() + &self.external_item_id),
                self.ssc.to_string())
            .arg("-A")
            .arg(generate_user_agent())
            .arg("--http1.1")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| error!("There is an issue in the cURL: {}", e))
            .unwrap();

        let i1_regex = Regex::new(r"<!--[ ]*-->")
            .map_err(|e| error!("There has been a very unexpected error in the regex creation (regex#1): {}", e))
            .unwrap();

        let raw_curl_result = curl.wait_with_output()
            .map_err(|e| error!("There has been an error with the curl stdout fetch: {}", e))
            .unwrap();

        let curl_result = str::from_utf8(&raw_curl_result.stdout)
            .map_err(|e| error!("There has been an error in the curl out aggregation: {}", e))
            .unwrap();


        let intermediate_result = i1_regex.replace_all(curl_result, "");

        let re = Regex::new(r#"<span class=\"productDetail__productPrice\"[<>a-zA-Z\= \"_0-9\/,!-]*>([0-9]*<\/span>[,.][0-9 ]*)€<\/span>"#)
            .map_err(|e| error!("There has been a very unexpected error in the main regex creation: {}", e))
            .unwrap();

        let re_res = match re.captures(&intermediate_result) {
            Some(e) => e,
            None => {
                error!("There has been an issue in the capturing group");
                panic!("see log");
            }
        };

        let output = match re_res.get(1) {
            Some(e) => e.as_str(),
            None => ""
        }.replace(",", ".").replace("</span>", "").replace(" ", "");

        trace!("DEBUG: result of that shit: {}", output);

        let res = output.parse::<f64>()
            .map_err(|e| error!("There has been an error in the result parsing: {}", e)).unwrap();

        return Ok(res);
    }

    fn get_casino(&self) -> BRes<f64> {
        trace!("Casino => trying to dig back the shop id");

        let complete_shop_id: Option<String> = {
            let mut out: Option<String> = None;

            for cookie in self.ssc.to_string().split(",") {
                let raw = base64::decode(&cookie).map_err(|e| error!("There has been an issue in the base64 cookie parsing: {}", e)).unwrap();
                let temp = str::from_utf8(&raw).map_err(|e| error!("There has been an issue in the base64 result accumulation: {}", e)).unwrap();

                if temp.starts_with("shopId") {
                    let raw_sid_split = temp.split("=").collect::<Vec<&str>>();

                    if raw_sid_split.len() == 2 {
                        out = Some(String::from(raw_sid_split[1]));
                    }
                }
            }

            out
        };


        let e_sid: String = match complete_shop_id {
            Some(intermediate_id) => {
                let temporary_split = intermediate_id.split("&").collect::<Vec<&str>>();

                if temporary_split.len() == 2 {
                    String::from(temporary_split[0])
                } else {
                    error!("The id of the shop doesn't seem to be contained inside the provided cookie. Here's more traceback ssc={} intermediate_id={}", &self.ssc, &intermediate_id);
                    panic!("See tracelog above");
                }
            },
            None => {
                error!("No complete_shop_id has been extracted. Here is the ssc: {}", &self.ssc);
                panic!("See tracelog above");
            }
        }; // external shop id


        let e_pid = self.external_item_id.to_string(); // external product id


        let prepared_url = format!("https://www.casino.fr/casinoexpress_web/affichageDetailProduit/{}{}", e_sid, e_pid);

        trace!("Issuing a request to casino.fr, using url:\n   {}", &prepared_url);
        let curl = wrap_ssc(Command::new("curl")
            .arg(prepared_url),
                self.ssc.to_string())
            .arg("-A")
            .arg(generate_user_agent())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| error!("There is an issue in the cURL: {}", e))
            .unwrap();

        let midway_raw = curl.wait_with_output()
                .map_err(|e| error!("There has been an error in the curl stdout fetch: {}", e)).unwrap();
        let raw = str::from_utf8(&midway_raw.stdout)
                .map_err(|e| error!("There has been an error in the curl out aggregation: {}", e)).unwrap();

        let re = Regex::new(r#"itemprop=\"price\">[ \t\n]+([0-9]+,[0-9*]+).*euro"#)
                .map_err(|e| error!("There has been an error in the regex: {}", e)).unwrap();

        let re_res = match re.captures(raw) {
            Some(e) => e,
            None => {
                error!("There has been an issue in the capturing group");
                panic!("see log");
            }
        };

        let output = match re_res.get(1) {
            Some(e) => e.as_str(),
            None => ""
        }.replace(",", ".");

        println!("Output: {}", output);

        let res = output.parse::<f64>()
            .map_err(|e| error!("There has been an error in the result parsing: {}", e)).unwrap();

        return Ok(res);

    }
}
