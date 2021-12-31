pub struct Quran {
    surahs : Vec<Surah>,
    qiraah : String,    // ? enum of Qiraat instead
    riwaya : String,    // ? enum
}

struct Surah {
    name: String,
    ayat: Vec<Ayah>,
    
}

struct Ayah {
    kalimat: Vec<String>,
    number: u32,

}

