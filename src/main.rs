use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;
use flate2::write::GzEncoder;
use flate2::Compression;



fn main(){

    if args().len()!=3{

        eprintln!("Usage: 'source' 'target'");
        return;
    }

    //above is the entry point of the program where it check the number
    //of arguments pass in the command line if its not equal to three then it
    //it prints an error message and return.


    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

  // in the above code the program first attempts to open the source file and then it 
  // is wrapped in Bufreader to optimize the reading and then unwrap() method is used to 
  // extract the underlying file.

  //secondly program create the target file

    let mut encoder = GzEncoder::new(output,Compression::default());

    // above code basiclly used to create a new GzEncoder which takes the output file and the default
    //compression level to compress the file.

    let start=Instant::now();
    //it marks the starting time of compression
    copy(&mut input, & mut encoder).unwrap();
    // the above copy function is used to read the data of source file==>
    //Bufreader and write it to GzEncoder

    let output= encoder.finish().unwrap();

    //above line shows that compression is finished ans that proe=cess is stopped.

    println!("length of the Source file: {:?}",input.get_ref().metadata().unwrap().len());
    println!("length of the Target file: {:?}",output.metadata().unwrap().len());
    println!("Elapsed time: {:?}",start.elapsed());

}