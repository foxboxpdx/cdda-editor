extern crate azul;
use azul::{prelude::*, widgets::{label::Label, button::Button}};
use std::fs::{File};
use std::io::{BufWriter, Write};
use cdda_editor::CDDASave;

// This thing's just a POC right now, so stuff is hard coded.
// Don't judge me!
fn main() { 
    let fname = "test.sav";
    let mut app = App::new(CDDAApp::new(fname), AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}

fn write(retval: CDDASave, outfile: String) {
   
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

// Define a datamodel wrapping a CDDASave
struct CDDAApp {
    save: CDDASave,
}

// Define Layout for the datamodel
impl Layout for CDDAApp {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new("Test").dom();
        let button = Button::with_label("HONK HONK").dom()
            .with_callback(On::MouseUp, toot);
        let muts = self.save.player.mutations.iter().map(|(k,v)|
            Label::new(format!("{} - {}", k, v.key)).dom()).collect();
        Dom::div().with_child(label).with_child(button).with_child(muts)
    }
}

impl CDDAApp {
    fn new(fname: &str) -> CDDAApp {
        let save = match CDDASave::from_file(fname) {
            Ok(x) => x,
            Err(_y) => { panic!("Unable to load sav file") }
        };
        CDDAApp { save: save }
    }
}

fn toot(_event: CallbackInfo<CDDAApp>) -> UpdateScreen {
    Redraw
}
