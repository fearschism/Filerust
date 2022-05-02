use std::fs::{File};
use std::io::{BufReader, BufRead};
pub struct Read {
    f:File
}

impl Read {
   pub fn new(ff:File)-> Self {
        Self {
            f: ff
        }
    }
    //reads ctx in file and stores it to a Vec<String>
    pub fn read_to_vec(&mut self)-> Vec<String> {
        let reader = BufReader::new(&self.f);
        reader.lines().map(|l|l.expect("Failed to read file")).collect()

    }
    //prints the value that has been read from the 'read_to_vec()'
    pub fn printer(list:&Vec<String>) {
        let mut counter =1;
        for i in list.iter() {
            println!("LINE#{}:|  {}",counter,*i); //reseves a refrence and derefrence to real value
            counter = &counter +1;
        }

    }

}
