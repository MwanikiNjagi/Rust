#[allow(unused)]

use std::fs::File;
use std::io::{BufReader, BufRead, Error, Write, ErrorKind};


fn main() {
    let path: &str = "lines.txt";
    let output: Result<File, Error> = File::create(path);
    let mut output = match output{
        Ok(file) => file,
        Err(error) =>{
            panic!("Problem creating file: {:?}", error);
        }    
    };
    write!(output, "Just Some\nRandom Words").expect("Failed to write to file");

    let input =  File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap());
    } 


    let output2: Result<File, Error> = File::create("rand.txt");
    let _output2 = match output2{
        Ok(file) => file,
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(_e) => panic!("Can't create file: {:?}", error),
            },

            _other_error => panic!("Problem opening file: {:?}", error),  
        }, 
    };
}
