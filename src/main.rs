extern crate clap;
use clap::{Arg, Command as ClapCommand};
use std::process::Command;

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

    // Parse command-line arguments
    let matches = app.get_matches();

    // Extract values of parsed arguments
    let input_files:&String = matches.get_one("input").unwrap();
    let output_file:&String = matches.get_one("output").unwrap();

    //Converting input file string to vector of input files
    let input_files: Vec<&str> = input_files.split(',').collect();

    //Using ffmpeg to concatenate input files into the output file
    // let mut cmd = Command::new("ffmpeg");
    // cmd.arg("-i").args(input_files).arg("-filter_complex").arg("concat=n=-2:v=1:a=1").arg(output_file);
    let mut cmd = Command::new("ffmpeg"); 
    cmd.arg("-i").arg(input_files[0]) // Take first input file
        .arg("-profile:v")
        .arg("baseline")
        // .. Add other HLS options
        .arg("index.m3u8"); // HLS playlist output

    match cmd.status(){
        Ok(exit_status)=>{
            if exit_status.success(){
                println!("Movies packaged successfully into: {}", output_file);
            } else {
                eprintln!("Error: Failed to create the package.");
            }
        }
        Err(e)=> eprintln!("Error: Failed to execute ffmpeg command: {}", e)
    }

}

