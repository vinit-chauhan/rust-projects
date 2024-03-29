pub mod toml_dynamic;

fn main() {
    let path: String = std::env::args()
        .nth(1)
        .expect("Unable to fetch 1st environment variable");
    toml_dynamic::read(&path)
}
