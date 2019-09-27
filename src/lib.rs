use serde_derive::{Serialize, Deserialize};
use serde_json::{Value};
use std::collections::HashMap;

// Define JSON data structure as a struct
#[derive(Serialize, Deserialize, Debug)]
pub struct CDDASave {
    pub turn: i64,
    pub calendar_start: i64,
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
    pub active_monsters: Vec<String>,
    pub stair_monsters: Vec<String>,
    pub kill_tracker: Kills,
    pub stats_tracker: Stats,
    pub player: Player,
    pub player_messages: PlayerMessages
}

// A struct for the kill tracker data
#[derive(Serialize, Deserialize, Debug)]
pub struct Kills {
    pub kills: HashMap<String, i32>,
    pub npc_kills: Vec<String>
}

// A struct for the stat tracker data
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub data: DataHash,
    pub initial_scores: Vec<String>
}

// A struct for the various datapoints of the stat tracker
#[derive(Serialize, Deserialize, Debug)]
pub struct DataHash {
    pub administers_mutagen: EventCounts,
    pub loses_addiction: EventCounts,
    pub gains_addiction: EventCounts,
    pub character_takes_damage: EventCounts,
    pub triggers_alarm: EventCounts,
    pub character_loses_effect: EventCounts,
    pub gains_skill_level: EventCounts,
    pub seals_hazardous_material_sarcophagus: EventCounts,
    pub character_gets_headshot: EventCounts,
    pub evolves_mutation: EventCounts,
    pub character_gains_effect: EventCounts,
    pub game_start: EventCounts,
    pub character_triggers_trap: EventCounts,
    pub gains_mutation: EventCounts,
    pub character_heals_damage: EventCounts,
    pub avatar_moves: EventCounts,
    pub character_kills_monster: EventCounts,
    pub throws_up: EventCounts
}

// EventCounts in the JSON is an array, with each element being
// an array containing 2 elements: a hash, and an int.  This......is
// difficult to represent.  Enums away!
#[derive(Serialize, Deserialize, Debug)]
pub struct EventCounts {
    pub event_counts: Vec<Vec<Value>>
}
#[derive(Serialize, Deserialize, Debug)]
pub enum EventEnum {
    Object(StatEvent),
    Number(i32)
}

// The possible types of events defined in event_counts
#[derive(Serialize, Deserialize, Debug)]
pub struct StatEvent {
    pub character: Option<Vec<String>>,
    pub technique: Option<Vec<String>>,
    pub add_type: Option<Vec<String>>,
    pub damage: Option<Vec<String>>,
    pub effect: Option<Vec<String>>,
    pub new_level: Option<Vec<String>>,
    pub skill: Option<Vec<String>>,
    pub from_trait: Option<Vec<String>>,
    pub to_trait: Option<Vec<String>>,
    pub avatar_id: Option<Vec<String>>,
    pub trap: Option<Vec<String>>,
    #[serde(rename = "trait")]
    pub _trait: Option<Vec<String>>,
    pub mount: Option<Vec<String>>,
    pub killer: Option<Vec<String>>,
    pub victim_type: Option<Vec<String>>
}

// A struct for player data
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub moves: i32,
    pub pain: i32,
    // Effects is kinda gross. It's a hashmap where the keys are
    // names of effects, but for some reason there's another hash
    // between that and the actual Effect part.
    pub effects: HashMap<String, HashMap<String, Effect>>,
    pub values: HashMap<String, String>,
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
    pub grab_resist: i32,
    pub throw_resist: i32,
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
    pub healthy: i32,
    pub healthy_mod: i32,
    pub healed_24h: Vec<i32>,
    pub thirst: i32,
    pub hunger: i32,
    pub fatigue: i32,
    pub sleep_deprivation: i32,
    pub stored_calories: i64,
    pub radiation: i32,
    pub stamina: i64,
    pub underwater: bool,
    pub oxygen: i32,
    pub traits: Vec<String>,
    pub mutations: HashMap<String, Mutation>,
    pub my_bionics: Vec<String>,
    pub skills: HashMap<String, Skill>,
    pub power_level: i32,
    pub max_power_level: i32,
    pub posx: i32,
    pub posy: i32,
    pub posz: i32,
    pub stim: i32,
    pub last_sleep_check: i64,
    pub pkill: i32,
    pub tank_plut: i32,
    pub reactor_plut: i32,
    pub slow_rad: i32,
    pub scent: i32,
    pub body_wetness: Vec<i32>,
    pub male: bool,
    pub cash: i64,
    pub recoil: f32,
    pub in_vehicle: bool,
    pub id: i32,
    pub hp_cur: Vec<i32>,
    pub hp_max: Vec<i32>,
    pub damage_bandaged: Vec<i32>,
    pub damage_disinfected: Vec<i32>,
    pub ma_styles: Vec<String>,
    pub addictions: Vec<String>,
    pub followers: Vec<String>,
    pub known_traps: Vec<Trap>,
    pub worn: Vec<Clothing>,
    pub activity_vehicle_part_index: i32,
    pub inv: Vec<Inventory>,
    pub weapon: Inventory,
    pub last_target_pos: Value,
    pub faction_warnings: Vec<String>,
    pub ammo_location: HashMap<String, String>,
    pub camps: Vec<String>,
    pub profession: String,
    pub scenario: String,
    pub controlling_vehicle: bool,
    pub grab_point: Vec<i32>,
    pub grab_type: String,
    pub focus_pool: i32,
    pub style_selected: String,
    pub keep_hands_free: bool,
    pub move_mode: String,
    pub magic: Magic,
    pub str_upgrade: i32,
    pub dex_upgrade: i32,
    pub int_upgrade: i32,
    pub per_upgrade: i32,
    pub activity: HashMap<String, String>,
    pub backlog: Vec<String>,
    pub temp_cur: Vec<i32>,
    pub temp_conv: Vec<i32>,
    pub frostbite_timer: Vec<i32>,
    pub learned_recipes: Vec<String>,
    pub items_identified: Vec<String>,
    pub vitamin_levels: HashMap<String, i32>,
    pub stomach: GutContents,
    pub guts: GutContents,
    pub translocators: HashMap<String, Vec<String>>,
    pub morale: Vec<MoraleModifier>,
    pub active_mission: i32,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
    pub failed_missions: Vec<String>,
    pub show_map_memory: bool,
    pub assigned_invlet: Vec<String>,
    pub invcache: Vec<String>
}

// A struct for player messages
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMessages {
    pub messages: Vec<Msg>,
    pub curmes: i64
}

// A struct for messages themselves
#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub turn: i64,
    pub message: String,
    pub count: i32,
    #[serde(rename = "type")]
    pub _type: i32
}

// A struct for the player effects
#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    pub eff_type: String,
    pub duration: i32,
    pub bp: i32,
    pub permanent: bool,
    pub intensity: i32,
    pub start_turn: i64
}

// A struct for player mutations
#[derive(Serialize, Deserialize, Debug)]
pub struct Mutation {
    pub key: i32,
    pub charge: i32,
    pub powered: bool
}

// A struct for player skills
#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub level: i32,
    pub exercise: i32,
    pub istraining: bool,
    pub lastpracticed: i64,
    pub highestlevel: i64
}

// A struct for known traps
#[derive(Serialize, Deserialize, Debug)]
pub struct Trap {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub trap: String
}

// A struct for player clothing
#[derive(Serialize, Deserialize, Debug)]
pub struct Clothing {
    pub typeid: String,
    pub bday: Option<i32>,
    pub damaged: Option<i32>,
    pub last_rot_check: i32,
    pub last_temp_check: i32,
    pub item_tags: Option<Vec<String>>,
    pub owner: String,
    pub relic_data: Value
}

// A struct for player inventory with optional fields.  Can be
// recursive, hope Rust is okay with that...
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Magic {
    mana: i32,
    spellbook: Vec<String>
}

// A struct for stomach/guts
#[derive(Serialize, Deserialize, Debug)]
pub struct GutContents {
    vitamins: HashMap<String, i32>,
    vitamins_absorbed: HashMap<String, i32>,
    calories: i32,
    water: String,
    max_volume: String,
    contents: String,
    last_ate: i64
}

// A struct for player morale
#[derive(Serialize, Deserialize, Debug)]
pub struct MoraleModifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub bonus: i32,
    pub duration: i32,
    pub decay_start: i32,
    pub age: i32
}
