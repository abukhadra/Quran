
mod config;
mod parser;
mod util;

use std::fs;
use config::get_config;

fn main() {

    let content = fs::read_to_string(get_config("QURAN_FILENAME"))
        .expect("could not read Quran data file");
    let ast = parser::run(content);

}
