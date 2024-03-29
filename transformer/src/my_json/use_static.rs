use colorize::AnsiColor;
use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct Sale {
    date: u32,
    id: String,
    product_id: u32,
    quantity: f64,
    unit: String,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct Product {
    category: String,
    name: String,
    id: u32,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

pub fn read_and_update(input_path: &str, output_path: &str) {
    let mut sales_and_products: SalesAndProducts = serde_json::from_str(
        std::fs::read_to_string(input_path)
            .expect("Unable to read input file".red().as_str())
            .as_str(),
    )
    .expect("Unable to Deserialize JSON file");

    sales_and_products.sales[1].quantity += 1.5;

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&sales_and_products).expect("Unable to Serialize JSON"),
    )
    .expect("Unable to write updated record to file");

    println!("{}", "Done!".green());
}
