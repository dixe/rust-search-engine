use std::collections::HashMap as HashMap;

use crate::index::index_types::*;
use crate::index::property_map::{PropertyMap};

pub struct SearchableIndex {


    next_id: usize,


    // Use a map for each different type
    // mapping from strings (words/tokens) to document ids

    properties_text_word_map: PropertyMap<TextT>,

    properties_integer_word_map: PropertyMap<IntegerT>,

    properties_sortable_text_word_map: PropertyMap<SortableTextT>,


    // Documents in index

    documents: HashMap<usize, ProcessedDocument>,
}


impl SearchableIndex {
    pub fn empty() -> Self {
        SearchableIndex {
            next_id: 1,
            properties_sortable_text_word_map: PropertyMap::new(),
            properties_text_word_map: PropertyMap::new(),
            properties_integer_word_map: PropertyMap::new(),

            documents: HashMap::new()

        }
    }

    fn increment_id(&mut self)  {
        self.next_id += 1;
    }

    fn insert_property(&mut self, prop: &IndexProperty ) {

        match &prop.data {
            PropertyType::Integer(data) => self.insert_integer_property(&prop.name, *data),
            PropertyType::Text(text) => self.insert_text_property(&prop.name, &text),
            PropertyType::SortableText(text) => panic!("Not implemted int property"),
        }


    }

    fn process_text_data(doc_id: usize, text: &str) -> HashMap::<&str, WordFrequency> {
        let mut word_freqs = HashMap::new();

        for word in text.split_whitespace() {
            if !word_freqs.contains_key(word) {
                word_freqs.insert(word, WordFrequency {doc_id, frequency: 0} );
            }

            if let Some(wf) = word_freqs.get_mut(&word) {
                wf.frequency += 1;
            }

        }

        word_freqs
    }

    fn insert_text_property(&mut self, name: &str, text: &str)
    {

        let word_freqs = SearchableIndex::process_text_data(self.next_id, text);

        // insert word_freqs into self hashmap for the given property and word
        for kv in word_freqs.iter() {
            let word = *kv.0;

            self.properties_text_word_map.insert_data(&name, word.to_string(), *kv.1);
        }

    }


    fn insert_integer_property(&mut self, name: &str, data: u32)
    {
        self.properties_integer_word_map.insert_data(name, data, WordFrequency { doc_id: self.next_id, frequency: 1});
    }


    pub fn from_documents(docs: &Vec::<ProcessedDocument>) -> Self {
        let mut result = SearchableIndex::empty();

        // indexing. for now just split on whitespace and call it a day, only index string and all in same map

        for doc in docs {
            for prop in doc.properties.iter() {
                result.insert_property(prop);
            }

            result.documents.insert(result.next_id, (*doc).clone());

            result.increment_id();
        }



        result


    }

    pub fn get_property_map_text(&self, name: &str) -> &HashMap<TextT, Vec::<WordFrequency>> {
        self.properties_text_word_map.get_map(name)
    }


    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}



#[derive(Clone)]
pub struct ProcessedDocument { // Represents a document with stopwords removed ect.
    properties: Vec::<IndexProperty>
}

impl ProcessedDocument {
    pub fn new(properties: Vec::<IndexProperty>) -> Self {
        // TODO remove stop words ect from this
        ProcessedDocument {
            properties
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    fn create_index() -> SearchableIndex {

        let documents = vec! {
            ProcessedDocument {
                properties: vec! {
                    IndexProperty {
                        name: "content".to_string(),
                        data: PropertyType::Text("lorup ipsum content for you needs. With lorup repeats".to_string())
                    },

                    IndexProperty {
                        name: "count".to_string(),
                        data: PropertyType::Integer(10)
                    }
                },
            }

        };

        SearchableIndex::from_documents(&documents)
    }


    #[test]
    fn document_count() {
        let index = create_index();

        assert_eq!(index.document_count(), 1);
    }
}
