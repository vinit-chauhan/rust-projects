use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    pub date: i64,
    pub id: u32,
    pub product_id: u32,
    pub quantity: f64,
    pub unit: String,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub category: String,
    pub name: String,
    pub id: u32,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SalesAndProducts {
    pub products: Vec<Product>,
    pub sales: Vec<Sale>,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SalesWithProduct {
    pub prod_category: String,
    pub prod_name: String,
    pub sale_date: i64,
    pub sale_quantity: f64,
    pub sale_unit: String,
}
