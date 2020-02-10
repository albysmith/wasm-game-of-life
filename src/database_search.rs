use wasm_bindgen::prelude::*;

use std::fs;
use std::fs::File;
use std::io::Write;

// mod toml_parse;
use crate::toml_parse::*;

// #[macro_use]
// mod print_macro;
// use crate::print_macro;
// use print_macro::console_log;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SearchItem {
    id: String,
    value: String,
}

// #[wasm_bindgen]
// pub struct SearchResults {
//     pub results: Vec<SearchItem>,
// }

#[wasm_bindgen]
impl SearchItem {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

pub struct Database {
    pub data: Vec<Object>,
}

pub struct Object {
    pub id: String,
    pub name: String,
    pub volume: u32,
}

impl Database {
    pub fn new() -> Database {
        Database { data: vec![] }
    }
    pub fn add(&mut self, object: Object) {
        self.data.push(object)
    }
}

impl Object {
    pub fn new(id: &str, name: &str, volume: u32) -> Object {
        Object {
            id: id.to_string(),
            name: name.to_string(),
            volume: volume,
        }
    }
}

fn search_loop(toml: Toml, search_token: &str) -> Vec<SearchItem> {
    let mut results = vec![];

    for item in toml.Ships.unwrap().iter() {
        if item
            .owner
            .clone()
            .unwrap_or_default()
            .contains(search_token)
        {
            results.push(SearchItem {
                id: item._id.clone().unwrap_or_default(),
                value: item.owner.clone().unwrap_or_default(),
            });
            continue;
        }
        if item.ship.clone().unwrap_or_default().contains(search_token) {
            results.push(SearchItem {
                id: item._id.clone().unwrap_or_default(),
                value: item.ship.clone().unwrap_or_default(),
            });
            continue;
        }
        if item
            .r#macro
            .clone()
            .unwrap_or_default()
            .contains(search_token)
        {
            results.push(SearchItem {
                id: item._id.clone().unwrap_or_default(),
                value: item.r#macro.clone().unwrap_or_default(),
            });
            continue;
        }
        if item._id.clone().unwrap_or_default().contains(search_token) {
            results.push(SearchItem {
                id: item._id.clone().unwrap_or_default(),
                value: item._id.clone().unwrap_or_default(),
            })
        }
    }
    results
}

fn get_toml() -> Toml {
    let toml_parsed: Toml = toml::from_str(include_str!("print_universe.txt")).expect("parsing toml");
    toml_parsed
}

#[wasm_bindgen]
pub fn search(search_token: String) -> String {
    let data = get_toml();
    let results = search_loop(data, &search_token);
    let result = format!("{:#?}", results);
    result
}
