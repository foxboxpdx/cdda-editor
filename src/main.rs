// It doesn't really do much right now, does it?
use cdda_editor::CDDASave;

fn main() { 
    let _loaded = match CDDASave::from_file("test.sav") {
        Ok(x) => x,
        Err(_e) => { panic!("Error loading save"); }
    };
    println!("It did a thing wow");
}

