use wasm_bindgen::prelude::*;



use std::fs::File;
use std::fs;
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

fn get_toml (value: String) -> String {
    let toml_str = include_str!("print_universe.txt");
    let toml_parsed: Toml = toml::from_str(&toml_str).expect("parsing toml");
    let mut results = vec![];
    for item in toml_parsed.Ships.unwrap().iter() {
        if item.owner.clone().unwrap_or_default().contains(&value) {
            results.push(SearchItem {id: item._id.clone().unwrap_or_default(), value: item.owner.clone().unwrap_or_default()});
            continue
        }
        if item.ship.clone().unwrap_or_default().contains(&value) {
            results.push(SearchItem {id: item._id.clone().unwrap_or_default(), value: item.ship.clone().unwrap_or_default()});
            continue
        }
        if item.r#macro.clone().unwrap_or_default().contains(&value) {
            results.push(SearchItem {id: item._id.clone().unwrap_or_default(), value: item.r#macro.clone().unwrap_or_default()});
            continue
        }
        if item._id.clone().unwrap_or_default().contains(&value) {
            results.push(SearchItem {id: item._id.clone().unwrap_or_default(), value: item._id.clone().unwrap_or_default()})
        }
    }
    let result = format!("{:#?}", results);
    result

}

#[wasm_bindgen]
pub fn search(value: String) -> String {
    let mut data = Database::new();
    // data.add(Object::new("5_4x42o", "Energy Cells", 10));
    // data.add(Object::new("5_jj30", "Advanced Electronics", 8));
    // data.add(Object::new("5_gvr8f", "Hull Parts", 40));
    // data.add(Object::new("5_gbt68d", "Claytronics", 1));
    // data.add(Object::new("5_bdfg4", "Advanced Composites", 5));
    let results = get_toml(value);
    let result = format!("{:#?}", results);
    result
}
