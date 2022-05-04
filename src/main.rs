#[macro_use]
extern crate serde;

mod config;

fn main() {
    let config = config::read_config();

    dbg!(config);
}
