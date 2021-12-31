use std::fmt;
use crate::util;


pub enum Token{ 
    Kalimah(String),
    SurahName(String),
    EndOfAyah(String),
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        match self{
            Token::Kalimah(kalimah) => {
                write!(fmt,"{} ",kalimah)
            },
            Token::SurahName(surah_name) => {
                writeln!(fmt,"\n\n=======================");
                writeln!(fmt,"سورة {}",surah_name);
                writeln!(fmt,"=======================")
            },
            Token::EndOfAyah(number) => {
                
                if number == "" { 
                    write!(fmt,"\n") 
                }
                else {
                    let number = util::to_eastern_arabic_numerals(number.to_string());
                    write!(fmt,"{{{}}}\n ",number)
                }
                

            },
        }        
    }
}