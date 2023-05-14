use std::str;

use std::process::{Command, Stdio};


// "<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*>[\n ]*([0-9\,]*)€[\n ]*<\/span>"
// "<span.+class=\"[a-zA-Z0-9\-\_ ]*product-card-price__price--final\"[a-zA-Z0-9\-\_ =\"]*>[\n ]*([0-9\,]*)€[\n ]*<\/span>"

fn main() {
    let grep = Command::new("echo")
            .arg("\"<span.+class=\\\"[a-zA-Z0-9\\-\\_ ]*product-card-price__price--final\\\"[a-zA-Z0-9\\-\\_ =\\\"]*>[\\n ]*([0-9\\,]*)€[\\n ]*<\\/span>\"")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let output_raw = grep.wait_with_output().unwrap();
        println!("{}", str::from_utf8(&output_raw.stdout).unwrap())
}
