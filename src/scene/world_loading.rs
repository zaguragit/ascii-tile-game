use std::{sync::Mutex, fs::{read_dir, read_to_string}, path::Path, ops::Mul};

use engine::{AsciiSprite, RGB, rgb, UpdateResult, Loading, util::draw_text};

use simulation::world::{World, creature::{SpeciesTemplate, Diet, SpeciesMap, SpeciesID}};
use toml::value::Table;

use crate::TEXT_SIZE;

use super::world::create_world_scene;

#[repr(u8)]
#[derive(Debug)]
pub enum LoadingTask {
    GeneratingWorld,
    CreatingLife,
}

impl LoadingTask {
    fn get_label(&self) -> &str {
        match self {
            LoadingTask::GeneratingWorld => "Generating world",
            LoadingTask::CreatingLife => "Creating life",
        }
    }
}

pub struct LoadingState {
    pub task: LoadingTask,
}

const WORLD_SIZE: usize = 128 * 3;
const LEVELS: usize = 2;

pub fn create_world_loading_scene() -> Loading<LoadingState, World<WORLD_SIZE, LEVELS>> {
    let state = LoadingState {
        task: LoadingTask::GeneratingWorld,
    };
    Loading::new(state, get_char_at, load, on_loaded)
}

fn get_char_at(state: &LoadingState, x: usize, y: usize) -> AsciiSprite {
    const bg: RGB = rgb(0.1, 0.1, 0.1);
    const fg: RGB = rgb(0.8, 0.5, 0.5);

    let text = state.task.get_label();

    match draw_text(text, x, y, (TEXT_SIZE.0 - text.len()) / 2, TEXT_SIZE.1 / 2, bg, fg) {
        Some(c) => c,
        None => AsciiSprite { bg, fg: bg, index: 0 },
    }
}

fn load(state: &Mutex<LoadingState>) -> World<WORLD_SIZE, LEVELS> {
    let seed = 5344545;
    let mut world = World::<WORLD_SIZE, LEVELS>::generate(seed);
    state.lock().unwrap().task = LoadingTask::CreatingLife;
    let mut species = load_species_templates(Path::new("assets/species"));
    let player_species_id = SpeciesID(species.keys().len() as u64);
    species.insert(player_species_id, load_player_template(Path::new("assets/player.toml")));
    world.create_life(seed, species, player_species_id);
    world
}

fn on_loaded(world: World<WORLD_SIZE, LEVELS>) -> UpdateResult {
    UpdateResult::SwitchScene(Box::new(create_world_scene(world)))
}

fn load_species_templates(directory: &Path) -> SpeciesMap {
    let species = read_dir(directory)
        .expect("Missing species directory")
        .enumerate()
        .map(|(i, res)| {
            let entry = res.unwrap();
            (SpeciesID(i as u64), load_species_template(entry.path().as_path()))
        });
    SpeciesMap::from_iter(species)
}

fn load_species_template(path: &Path) -> SpeciesTemplate {
    let table: Table = read_to_string(path).unwrap().as_str().parse::<Table>().expect("Couldn't parse species file");
    SpeciesTemplate {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        symbol: table["symbol"].as_str()
            .and_then(|x| x.bytes().nth(0))
            .and_then(|x| Some(x as char))
            .or(Some('_')).unwrap(),
        diet: {
            let t = table["diet"].as_table();
            Diet {
                meat: t.and_then(|t| t["meat"].as_bool()).or(Some(false)).unwrap(),
                plants: t.and_then(|t| t["plants"].as_bool()).or(Some(false)).unwrap(),
                light: t.and_then(|t| t["light"].as_bool()).or(Some(false)).unwrap(),
            }
        },
        max_health: table["body"]["max_health"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        max_nutrition: table["body"]["max_nutrition"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        smelliness: table["body"]["smelliness"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        strength: table["body"]["strength"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        awareness: table["behavior"]["awareness"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        curiosity: table["behavior"]["curiosity"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        friendliness: table["behavior"]["friendliness"].as_float().or(Some(0.0)).unwrap().mul(127.0) as i8,
    }
}


fn load_player_template(path: &Path) -> SpeciesTemplate {
    let table: Table = read_to_string(Path::new(path)).unwrap().as_str().parse::<Table>().expect("Couldn't parse species file");
    SpeciesTemplate {
        name: "Player".to_string(),
        symbol: 1 as char,
        diet: {
            let t = table["diet"].as_table();
            Diet {
                meat: t.and_then(|t| t["meat"].as_bool()).or(Some(false)).unwrap(),
                plants: t.and_then(|t| t["plants"].as_bool()).or(Some(false)).unwrap(),
                light: t.and_then(|t| t["light"].as_bool()).or(Some(false)).unwrap(),
            }
        },
        max_health: table["body"]["max_health"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        max_nutrition: table["body"]["max_nutrition"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        smelliness: table["body"]["smelliness"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        strength: table["body"]["strength"].as_float().or(Some(0.0)).unwrap().mul(255.0) as u8,
        awareness: 0,
        curiosity: 0,
        friendliness: 0,
    }
}