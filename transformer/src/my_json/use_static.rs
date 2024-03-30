use crate::types::sales_and_products::SalesAndProducts;
use colorize::AnsiColor;

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
