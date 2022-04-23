use serde_derive::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{BufReader, BufRead, Error, BufWriter, Write};
use serde_json::{Value};
use std::collections::HashMap;

// Initialize a CDDASave struct from a file
impl CDDASave {
    pub fn from_file(fname: &str) -> Result<CDDASave, Error> {
        let f = File::open(fname)?;
        let reader = BufReader::new(f);
        // First line is a comment
        let mut data = String::new();
        let mut lines_iter = reader.lines();
        if let Some(_header) = lines_iter.next() { }
        
        for line in lines_iter {
            let l = line?;
            data.push_str(&l);
        }
        let retval: CDDASave = match serde_json::from_str(&data) {
            Ok(x) => x,
            Err(e) => {
                println!("Error parsing savefile JSON: {}", e);
                return Err(Error::new(std::io::ErrorKind::Other, "oh no!"));
            }
        };
        Ok(retval)
    }
    
    pub fn write(&self, outfile: String) {
   
        // Reserialize
        let ser = match serde_json::to_string(&self) {
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
}

// Define JSON data structure as best as possible
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CDDASave {
    pub turn: i64,
    pub calendar_start: i64,
    pub game_start: i64,
    pub initial_season: i32,
    pub auto_travel_mode: bool,
    pub run_mode: i32,
    pub mostseen: i32,
    pub levx: i32,
    pub levy: i32,
    pub levz: i32,
    pub om_x: i32,
    pub om_y: i32,
    pub grscent: String, // This is some space-separated Vec of i32s?
    pub typescent: String,
    pub active_monsters: Value, // Simplify these four into just generic Value types
    pub stair_monsters: Value,  // as there is no reason to be parsing and manipulating them
    pub kill_tracker: Value,    // at least for initial versions
    pub stats_tracker: Value,   // Especially since stats has some horrendous Vec<Vec<HashMap>> data structure
    pub player: Player,
    pub player_messages: Value, // Simplify also
}

// A struct for player data
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Player {
    pub moves: i32,
    pub pain: i32,
    pub effects: Value, // Simplify for now!
    pub damage_over_time_map: Value, // Need further research
    pub values: HashMap<String, String>, // I hope this is all it is ugh
    pub blocks_left: i32,
    pub dodges_left: i32,
    pub num_blocks_bonus: i32,
    pub num_dodges_bonus: i32,
    pub armor_bash_bonus: i32,
    pub armor_cut_bonus: i32,
    pub speed: i32,
    pub speed_bonus: i32,
    pub dodge_bonus: f32,
    pub block_bonus: i32,
    pub hit_bonus: f32,
    pub bash_bonus: i32,
    pub cut_bonus: i32,
    pub bash_mult: f32,
    pub cut_mult: f32,
    pub melee_quiet: bool,
    pub throw_resist: i32,
    pub last_updated: i64,
    pub body: BodyInfo,
    pub posx: i32,
    pub posy: i32,
    pub posz: i32,
    pub str_cur: i32,
    pub str_max: i32,
    pub dex_cur: i32,
    pub dex_max: i32,
    pub int_cur: i32,
    pub int_max: i32,
    pub per_cur: i32,
    pub per_max: i32,
    pub str_bonus: i32,
    pub dex_bonus: i32,
    pub per_bonus: i32,
    pub int_bonus: i32,
    pub base_age: i32,
    pub base_height: i32,
    pub blood_type: String,
    pub blood_rh_factor: bool,
    pub custom_profession: String,
    pub healthy: i32,
    pub healthy_mod: i32,
    pub thirst: i32,
    pub hunger: i32,
    pub fatigue: i32,
    pub activity_history: HashMap<String, Value>, // Mostly string/i32 but there's a bool in there
    pub sleep_deprivation: i32,
    pub stored_calories: i64,
    pub radiation: i32,
    pub stamina: i64,
    pub vitamin_levels: HashMap<String, i32>,
    pub pkill: i32,
    pub omt_path: Value, // Need further research
    pub consumption_history: Value, // A vec of hashmaps
    pub destination_activity: Value, // These four look to be hashmaps with a single key of 'type'
    pub activity: Value,             // with a value of ACT_NULL but need further research
    pub stashed_outbounds_activity: Value,
    pub stashed_outbounds_backlog: Value,
    pub backlog: Value, // Need further research
    pub activity_vehicle_part_index: i32,
    pub stim: i32,
    pub type_of_scent: String,
    pub underwater: bool,
    pub oxygen: i32,
    pub traits: Vec<String>,
    pub mutations: HashMap<String, Mutation>,
    pub magic: Magic,
    pub martial_arts_data: MartialArts,
    pub my_bionics: Vec<Bionic>,
    pub move_mode: String,
    pub morale: Vec<MoraleModifier>,
    pub skills: HashMap<String, Skill>,
    pub proficiencies: ProficiencyData,
    pub power_level: String,  // This used to be an i32...
    pub max_power_level: i32,
    pub stomach: GutContents,
    pub guts: GutContents,
    pub automoveroute: Vec<Value>, // Need further research
    pub known_traps: Value, // Yeah just simplify this to generic Value for now. Absolutely MASSIVE though.
    pub last_sleep_check: i64,
    pub slow_rad: i32,
    pub scent: i32,
    pub male: bool,
    pub cash: i64,
    pub recoil: f32, // For some reason this is parsing as a float instead of an int?
    pub in_vehicle: bool,
    pub id: i32,
    pub addictions: Vec<Value>, // Need further research
    pub followers: Vec<Value>,  // Need further research
    pub worn: Vec<Clothing>,
    pub inv: Vec<Value>, // Need further research
    pub weapon: Option<Value>, // Huge mess
    pub last_target_pos: Value, // There is no null in Rustlang
    pub destination_point: Value, // See above
    pub faction_warnings: Vec<Value>, // Need further research
    pub camps: Vec<Value>, // Need further research
    pub profession: String,
    pub scenario: String,
    pub controlling_vehicle: bool,
    pub grab_point: Vec<i32>,
    pub grab_type: String,
    pub focus_pool: i64,
    pub str_upgrade: i32,
    pub dex_upgrade: i32,
    pub int_upgrade: i32,
    pub per_upgrade: i32,
    pub learned_recipes: Vec<String>,
    pub items_identified: Vec<String>,
    pub translocators: Value, // Need further research
    pub active_mission: i32,
    pub active_missions: Vec<i32>,
    pub completed_missions: Vec<i32>,
    pub failed_missions: Vec<i32>,
    pub show_map_memory: bool,
    pub assigned_invlet: Vec<Value>, // Need further research
    pub invcache: Vec<Value>, // Need further research
    pub calorie_diary: Vec<Value>,  // Vec of hashes - spent, gained, activity(another vec)
    pub preferred_aiming_mode: String
}

// Data about proficiencies
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProficiencyData {
    pub known: Vec<Value>, // I assume these two hash keys are the same data
    pub learning: Vec<Value> // type but need further research
}

// Info about martial arts
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MartialArts {
    pub ma_styles: Vec<String>,
    pub keep_hands_free: bool,
    pub style_selected: String
}

// Info about body part current status
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BodyInfo {
    pub head: BodyPartStatus,
    pub eyes: BodyPartStatus,
    pub mouth: BodyPartStatus,
    pub torso: BodyPartStatus,
    pub arm_l: BodyPartStatus,
    pub arm_r: BodyPartStatus,
    pub hand_l: BodyPartStatus,
    pub hand_r: BodyPartStatus,
    pub leg_l: BodyPartStatus,
    pub foot_l: BodyPartStatus, // Yes i know this isn't the same order as arm/hand but
    pub leg_r: BodyPartStatus,  // this is how it appears in the save file
    pub foot_r: BodyPartStatus
}

// Details of body info for each tracked body part
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BodyPartStatus {
    pub id: String,
    pub hp_cur: i32,
    pub hp_max: i32,
    pub damage_bandaged: i32,
    pub damage_disinfected: i32,
    pub wetness: i32,
    pub temp_cur: i32,
    pub temp_conv: i32,
    pub frostbite_timer: i32
}

// A struct for player mutations
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mutation {
    pub key: i32,
    pub charge: i32,
    pub powered: bool
}

// A struct for player skills
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skill {
    pub level: i32,
    pub exercise: i32,
    pub istraining: bool,
    pub lastpracticed: i64,
    pub highestlevel: i64
}

// A struct for player bionics
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Bionic {
    pub id: String,
    pub invlet: i32,
    pub powered: bool,
    pub charge: i32,
    pub ammo_loaded: String,
    pub ammo_count: i32
}

// A struct for player clothing
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Clothing {
    pub typeid: String,
    pub bday: Option<i32>,
    pub owner: String,
    pub last_temp_check: i32,
    pub item_tags: Option<Vec<String>>,
    pub components: Option<Value>, // Big mess
    pub contents: Option<Value> // Another big mess and also recursive
}

// A struct for player inventory with optional fields.  Can be
// recursive, hope Rust is okay with that...
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Inventory {
    pub typeid: String,
    pub charges: Option<i32>,
    pub bday: Option<i64>,
    pub item_vars: Option<HashMap<String, String>>,
    pub last_rot_check: Option<i32>,
    pub last_temp_check: Option<i32>,
    pub contents: Option<Vec<Inventory>>,
    pub components: Option<Vec<Inventory>>,
    pub item_tags: Option<Vec<String>>,
    pub owner: Option<String>,
    pub specific_energy: Option<i64>,
    pub temperature: Option<i64>,
    pub active: Option<bool>,
    pub relic_data: Value
}

// A struct for player magic
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Magic {
    pub mana: i32,
    pub spellbook: Vec<Spell>,
    pub invlets: Value  // Empty hash?
}

// Magic spells
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Spell {
    pub id: String,
    pub xp: i32
}

// A struct for stomach/guts
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GutContents {
    vitamins: HashMap<String, i32>,
    calories: i32,
    water: String,
    max_volume: String,
    contents: String,
    last_ate: i64
}

// A struct for player morale modifiers
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MoraleModifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub item_type: Option<String>,
    pub bonus: i32,
    pub duration: i32,
    pub decay_start: i32,
    pub age: i32
}
