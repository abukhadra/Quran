use std::fs;
use crate::util::format;
use crate::parser;
use crate::config::get_config;

pub fn run() {

    let content = fs::read_to_string(get_config("QURAN_FILENAME"))
        .expect("could not read Quran data file");
    let quran = parser::run(content);
    for surah in quran.surahs {
        match &*(surah.name) {
            "الفاتحة" => {            
                println!("========= {} ========= ", &surah.name);
                println!("print each verse on a line:\n");         
                format::println_verses(&surah);                        
                println!("************");
            },
            "البقرة" => { 
                println!("========= {} ========= ", &surah.name);
                println!("number of ayat: {}", &surah.ayat.len());
                println!("fifth ayah : \n\t\t{}", &surah.ayat[4]);
                println!("************");
            },
            _ => {}
                
        }
    }
    // print the whole Quran
    // println!("{}", quran);  
}
