use colorize::AnsiColor;
use serde_json::{Number, Value};

pub fn read_and_update(input_path: &str, output_path: &str) {
    let file_text =
        std::fs::read_to_string(input_path).expect("Unable to read the file".b_red().as_str());

    let mut sales_and_products = file_text
        .parse::<serde_json::Value>()
        .expect("Unable to parse json file".b_red().as_str());

    let old_quantity = sales_and_products["sales"][1]["quantity"].clone();

    sales_and_products["sales"][1]["quantity"] =
        Value::Number(Number::from_f64(old_quantity.as_f64().unwrap() + 1.5).unwrap());

    let output_string = serde_json::to_string_pretty(&sales_and_products)
        .expect("Unable to serialize JSON".b_red().as_str());

    std::fs::write(output_path, output_string)
        .expect("Unable to write output to file".red().as_str());

    println!("{}", "Done!".b_green());
}
