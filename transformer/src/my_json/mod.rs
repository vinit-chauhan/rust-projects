pub mod use_dynamic;
pub mod use_static;

#[allow(unused)]
pub fn update_json(is_dynamic: bool) {
    if is_dynamic {
        use_dynamic::read_and_update(
            std::env::args()
                .nth(1)
                .expect("Unable to fetch 1st environment variable")
                .as_str(),
            std::env::args()
                .nth(2)
                .expect("Unable to fetch 2nd environment variable")
                .as_str(),
        )
    } else {
        use_static::read_and_update(
            std::env::args()
                .nth(1)
                .expect("Unable to fetch 1st environment variable")
                .as_str(),
            std::env::args()
                .nth(2)
                .expect("Unable to fetch 2nd environment variable")
                .as_str(),
        )
    }
}
