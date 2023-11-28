// Accessing External Crate
extern crate flate2;

//Accessing Flate libraries
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main(){
    // If arguments Length not equal to 3
    //
    if args().len() != 3{
        eprintln!("Usage: `source` `target`");
        return;
    }
    //Select source file and get the 1st nth argument 
    let mut input =  BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // Target file as 2nd nth argument
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // create an encoder 
    let mut encoder GzEncoder::new(output, Compression::default());
    //Start encoder- New instant
    let start = Instant::new();
    // copy data and encoder
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    //Getting source lenght
    println!(
        "Source len:{:?}",
        input.get_ref().metadata().unwrap.len();
    );
    //Getting the target lenght
    println!(
        "Target len:{:?}",
        output.metadata().unwrap.len();
    );
    //Time Taken
    println!("Elapsed time: {:?}", start.elapsed());
}