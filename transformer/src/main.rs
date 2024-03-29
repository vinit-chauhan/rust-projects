pub mod toml_dynamic;
pub mod toml_static;

fn main() {
    let path: String = std::env::args()
        .nth(1)
        .expect("Unable to fetch 1st environment variable");

    toml_dynamic::read(&path);
    toml_static::read(&path);
}
