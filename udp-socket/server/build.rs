extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/tuntap.c")
        .warnings(true)
        .compile("tuntap");
}
