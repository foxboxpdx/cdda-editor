use std::fs::{File};
use std::io::{BufReader, BufWriter, Write};
use cdda_editor::CDDASave;

// This thing's just a POC right now, so stuff is hard coded.
// Don't judge me!

fn main() {
    let testfile = "test.sav";
    let outfile = "processed.sav";

    let f = match File::open(testfile) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file {}: {}", testfile, e);
            return;
        }
    };
    let reader = BufReader::new(f);

    // Deserialize
    let retval: CDDASave = match serde_json::from_reader(reader) {
        Ok(x) => x,
        Err(e) => {
            println!("Error parsing savefile JSON: {}", e);
            return;
        }
    };
    
    // Reserialize
    let ser = match serde_json::to_string(&retval) {
        Ok(x) => x,
        Err(e) => {
            println!("Error serializing to JSON: {}", e);
            return;
        }
    };

    // Dump
    let ofile = match File::create(&outfile) {
        Ok(f) => f,
        Err(e) => {
            println!("Error creating output file {}: {}", &outfile, e);
            return;
        }
    };
    
    let mut ofile_writer = BufWriter::new(ofile);
    match ofile_writer.write_all(ser.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error writing file {}: {}", &outfile, e);
            return;
        }
    }
}
