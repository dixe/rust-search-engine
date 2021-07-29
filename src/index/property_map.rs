use std::collections::HashMap as HashMap;
use std::hash::Hash;
use std::fmt::Debug;


use crate::index::index_types::*;

pub struct PropertyMap<T> {
    map: HashMap<PropertyName, HashMap<T, Vec::<WordFrequency>>>,
}


impl<T: Clone + Eq + Hash + Debug> PropertyMap<T> {
    pub fn new() -> Self {
        PropertyMap {
            map: HashMap::new()
        }
    }

    fn get_or_create_mut(&mut self, key: &str) -> &mut HashMap<T, Vec::<WordFrequency>> {
        let k = key.to_string();
        if !self.map.contains_key(&k) {
            self.map.insert(k.clone(), HashMap::new());
        }
        self.map.get_mut(&k).unwrap()

    }

    pub fn get_map(&self, key: &str) -> &HashMap<T, Vec::<WordFrequency>> {
        self.map.get(key).unwrap() // TODO don't just unwrap
    }

    pub fn insert_data(&mut self, property: &str, data: T, freq: WordFrequency) {

        let property_map = self.get_or_create_mut(property);

        if !property_map.contains_key(&data) {
            property_map.insert(data.clone(), Vec::new());
        }


        let freq_map = property_map.get_mut(&data).unwrap();

        freq_map.push(freq)
    }
}
