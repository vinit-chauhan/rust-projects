use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    pub date: u32,
    pub id: String,
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
