extern crate clap;// parses CLI arguments
use clap::{Arg, Command as ClapCommand};
use std::{process::Command, fs};

fn main() {
    // Create the App builder
    let app = ClapCommand::new("Movie Packager")
        .version("1.0")
        .author("Faith Kimani")
        .about("Packaging movies using ffmpeg")
        // Define command-line arguments
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input movie files (comma-separated)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output packaged movie file")
                .required(true),
        );
        
    
    // Output directory (with error handling)
    let output_dir = "src/assets/outputs";
    match fs::create_dir_all(output_dir) {
        Err(e) => {
          eprintln!("Failed to create output dir: {}", e);
          std::process::exit(1);
        }
        Ok(_) => { // dir created 
        }
    }

    // Parse command-line arguments
    let matches = app.get_matches();

    // Extract values of parsed arguments
    let input_files:&String = matches.get_one::<String>("input").unwrap();
    let output_file = format!("{}/{}", output_dir, matches.get_one::<String>("output").unwrap());


    //Converting input file string to vector of input files(the .ts files)
    let input_files: Vec<&str> = input_files.split(',').collect();

    //Using ffmpeg to concatenate input files into the output file
    let mut cmd = Command::new("ffmpeg"); 
    cmd.arg("-i")
    .arg(input_files[0]) // Take first input file
    .arg("-c:v") 
    .arg("libx264")
    .arg("-hls_time")  
    .arg("10")
    .arg("-hls_list_size") 
    .arg("0")
    .arg("-f")
    .arg("hls")
    .arg(output_file); // HLS playlist output

    match cmd.status(){
        Ok(exit_status)=>{
            if exit_status.success(){
                println!("Movies packaged successfully into: {}", output_dir);
            } else {
                eprintln!("Error: Failed to create the package.");
            }
        }
        Err(e)=> eprintln!("Error: Failed to execute ffmpeg command: {}", e)
    }

}


