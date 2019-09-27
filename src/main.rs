use std::fs::{File};
use std::io::{BufReader};
use cdda_editor::CDDASave;

// This thing's just a POC right now, so stuff is hard coded.
// Don't judge me!

fn main() {
    let testfile = "test.sav";

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
}
