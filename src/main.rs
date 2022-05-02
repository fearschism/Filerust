pub mod Read;
use std::fmt::Error;
use std::path::Path;
use std::{fs, path};
use std::fs::File;
use std::io;
use colored::*;
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_imports)]
fn main(){
    loop { 
        println!("Please select a service");
        println!("1-Read");
        println!("2-Write");
        //generates input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read!");
       let mut input_trimmed = input.trim();
       match input_trimmed.parse::<i32>() {
           Ok(1) => {
              
               print!("\x1B[2J\x1B[1;1H"); //clears the previous prints in cmd
                let all_files:Vec<String> = fs::read_dir(".\\src\\All-files").expect("Dir is not valid").map(|dir_entry| dir_entry.unwrap().path().file_name().unwrap().to_str().unwrap().to_owned())
                .collect(); 
                let all_files_paths:Vec<String> = fs::read_dir(".\\src\\All-files").expect("Dir is not valid").map(|dir_entry| dir_entry.unwrap().path().display().to_string())
                .collect(); 
                println!("Please choose a file"); let mut count:u32=1;
                for fil in all_files {
                    println!("{}-Name: {}",&count.to_string().bright_green(),fil.magenta());
                    //std::io::stdin().read_line(&mut inp).expect("failed to read message");
                    count = &count +1;  
                }
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).expect("failed to read!");
                let _int:u32 = temp.trim().parse().expect("Failed to read as u32");
                print!("\x1B[2J\x1B[1;1H");
                if _int< count {
                let path_to_file = all_files_paths.get((&_int-1) as usize).unwrap();
                let pth = Path::new(path_to_file);
                let _Open_File = File::open(pth).expect("failed to locate file");
                let r = Read::Read::Read::new(_Open_File).read_to_vec();
                println!("Reading: {}",&pth.to_string_lossy().blue().bright_yellow());
                Read::Read::Read::printer(&r);
                let succ = "Reading has been successful".bright_green().bold();
                println!("{}",succ);
            }

                
            
           }    
           Ok(2) => {
            print!("\x1B[2J\x1B[1;1H");
           }
           Ok(_) => {println!("Wrong input");}
           Err(_) => {println!("Error numbers only");}
       }
        
       fn check(x:bool)->Result<bool,()> {
            Ok(x)
       }
    }
}