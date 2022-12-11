use std::fs::File;
use std::{env, io};
use std::io::{BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let mut args = env::args().into_iter();
    args.next(); // skip first arg given
    let f1_file_path = match args.next() {
        None => {
            println!("Missing first file name arguments, using default of csv1.csv");
            "csv1.csv".to_string()
        }
        Some(s) => {s}
    }; // get first filename
    let f2_file_path = match args.next() {
        None => {
            println!("Missing second file name arguments, using default of csv2.csv");
            "csv2.csv".to_string()
        }
        Some(s) => {s}
    }; // get second filename
    let output_file_name = match args.next() {
        None => {
            println!("No output filename given, using default of combined.csv");
            "combined.csv".to_string()
        }
        Some(s) => {s}
    }; // get output file name

    println!("File names used: {}, {}", f1_file_path, f2_file_path);

    // make an iterators for file 1 and file 2 that iterate on each line.
    let mut f1_iter = read_lines(f1_file_path).expect("File 1 invalid for reading lines").peekable();
    let mut f2_iter = read_lines(f2_file_path).expect("File 2 invalid for reading lines").peekable();

    // combine both files in "f1+,+f2" style
    let combined_contents: Vec<String> = {
        let mut out: Vec<String> = vec![];
        loop {
            let f1_line = match f1_iter.next() {
                None => {"".to_string()}
                Some(s) => {
                    match s {
                        Ok(str) => {str}
                        Err(_) => {"".to_string()}
                    }
                }
            }; // f1 line by line string
            let f2_line = match f2_iter.next() {
                None => {"".to_string()}
                Some(s) => {
                    match s {
                        Ok(str) => {str}
                        Err(_) => {"".to_string()}
                    }
                }
            }; // f2 line by line string
            let combined_line = {
                if f1_line.eq("") {
                    format!("{}\n",f2_line) // if the first line is empty, just output the second line
                } else if f2_line.eq("") {
                    format!("{}\n",f1_line) // if the second line is empty, just output the first line
                } else {
                    format!("{},{}\n",f1_line,f2_line) // if both lines are present, output the combination of them
                }
            }; // combine both f2line and f1line
            out.push(combined_line); // push the new line to the output vector
            if f1_iter.peek().is_none() && f2_iter.peek().is_none() {
                break;
            } // if both iterators are empty, break the loop
        }
        out
    }; // generate a vector that will be the desired combination of the two files

    let mut output_file = File::create(output_file_name).unwrap();
    for line in combined_contents {
        output_file.write_all(line.as_bytes()).expect("Unable to write to output file, possibly permissions error?");
    }
    output_file.flush().expect("Error unable to flush data buffer to output file, permissions error?");

}
