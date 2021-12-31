# Quran

# A Quran Library written in rust. 
- The goal of the project is to provide functions to help with the development of Islamic and Arabic applications  
- prefer data files over coding to allow other projects to benifit from the data and support for future platforms  
- Implementation will be done in rust to allow compilation and generation of native libraries that can be used effeciently from any other programming language.

## Status
Currently, only the lexer is completed, to run a quick test :  
    ````
    $  cargo run > output.txt
    ````

 this will reformat the Quran data file data/quran.txt and store the output to output.txt as follows:  
  1-  each verse will be printed on a separate line.  
  2- convert verse numbers from Arabic Numerals to Eastern Arabic Numerals.  
  3- add an ASCII frame to the Surah title. 
  
Next : build the lexical_analyzer to arrange the elements into an AST. 

## Current Todo items
- [x] generate Quran data file in text format
- [ ] data file transformation: transform to other formats such as json and xml 
- [ ] serialize the data 
- [ ] add functions to process the data


## Suggested Features
- [ ] Auto correction
- [ ] Auto completion 
- [ ] Data extraction and transformation 
- [ ] Tajweed
- [ ] Recognition 
- [ ] Intigration with other libraries and future projects, such as Arabic grammar , Tafseer, Hadeeth ...etc
- [ ] APIs
- [ ] Syntax highlighting 
- [ ] Tools

