use serde_json::{Number, Value};
use std::env;
use std::fs;

fn main() {
    let input_path = env::args().nth(1).expect("error reading commandline args");
    let output_path = env::args().nth(2).expect("error reading commandline args");

    let mut sales_and_products = {
        let sales_and_products_text = fs::read_to_string(input_path).expect("error reading file");

        serde_json::from_str::<Value>(&sales_and_products_text).expect("error serializing to JSON")
    };

    if let Value::Number(quantity) = &sales_and_products["sales"][1]["quantity"] {
        sales_and_products["sales"][1]["quantity"] =
            Value::Number(Number::from_f64(quantity.as_f64().unwrap() + 1.5).unwrap());
    }

    fs::write(
        output_path,
        serde_json::to_string_pretty(&sales_and_products).expect("error parsing to JSON"),
    )
    .expect("error writing to file");
}
