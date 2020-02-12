mod ken_all;

#[macro_use]
extern crate serde_derive;
use ken_all::config::Config;
use ken_all::database::Database;
use ken_all::ken_all::KenAll;

fn main() {
    let ken_all_vec = KenAll::read("KEN_ALL.CSV");
    let config = Config::read("config.toml");
    let database = Database::new(config.database);

    database.multiple_insert_ken_all(ken_all_vec);

    eprintln!("done!");
}
