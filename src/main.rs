use std::fs;

const QURAN_FILENAME : &str = "data/quran.txt";

fn main() {

    // for now we are just printing the quran text file to the screen
    let contents = fs::read_to_string(QURAN_FILENAME)
        .expect("could not read Quran data file");
    println!("{}", contents);
}
