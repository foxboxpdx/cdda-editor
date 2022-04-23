// Still needs a lot of work but the proof of concept is sound
use cdda_editor::CDDASave;
use clap::Parser;

// Use Clap crate for command-line stuff
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Print out current player info (skills, mutations)
    #[clap(short, long)]
    view: String,

    /// Filename to load save data from
    #[clap(short, long)]
    filename: String,
}

fn main() { 
    let args = Args::parse();

    match args.view.as_str() {
        "skills" => get_skills(&args.filename),
        "mutations" => get_mutations(&args.filename),
        "magic" => get_magic(&args.filename),
        "monsters" => get_monsters(&args.filename),
        _ => println!("Something idk")
    };
}

// This should return a result
fn load_savefile(fname: &str) -> CDDASave {
    let loaded = match CDDASave::from_file(fname) {
        Ok(x) => x,
        Err(_e) => { panic!("Error loading save"); }
    };
    loaded
}

fn get_skills(fname: &str) {
    println!("Called get_skills");
    let save = load_savefile(fname);
    let skills = &save.player.skills;
    for (s, t) in skills {
        println!("{}: {}", s, t.level);
    }
}

fn get_mutations(fname: &str) {
    println!("Called get_mitations");
    let save = load_savefile(fname);
    let mutations = &save.player.mutations;
    for m in mutations.keys() {
        println!("{}", m);
    }
}

fn get_magic(fname: &str) {
    println!("Called get_magic");
    let save = load_savefile(fname);
    let magic = &save.player.magic;
    for m in &magic.spellbook {
        println!("{} ({} xp)", m.id, m.xp);
    }
    println!("Mana: {}", magic.mana);
}

fn get_monsters(fname: &str) {
    println!("Called get_monsters");
    let save = load_savefile(fname);
    println!("Currently aware of {} active monsters", save.active_monsters.len());
}