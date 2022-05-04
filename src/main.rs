#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

mod config;

fn main() {
    let config = config::read_config().unwrap();

    dbg!(config);
}
