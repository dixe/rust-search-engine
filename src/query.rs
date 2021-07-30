use std::collections::HashSet as HashSet;

use crate::index::index_types::*;
use crate::index::searchable_index::{SearchableIndex};
use crate::search_result::{SearchResultIds};




// This is a processed query where a search query and maybe some facets are
// procesed to produce this query that the index querier can execute on an index


pub struct IndexQuery {
    criterion: Criterion,
    facets: Option<FacetQuery>

}

#[derive(Clone)]
pub struct PropertyQuery {
    name: PropertyName,
    query_data: PropertyType,
}


pub struct Criterion {
    properties: Vec::<PropertyQuery>,

}

pub enum FacetQueryType {
    And,
    Or
}

pub struct FacetQuery {
    query_type: FacetQueryType,
    criterion: Option<Criterion>,
    sub_queries: Vec::<FacetQuery>
}



pub fn query_index(index: &SearchableIndex, query: &IndexQuery) -> SearchResultIds {

    // Gather freq of each property query

    let mut doc_ids = HashSet::new();

    for p in &query.criterion.properties {

        let ids: HashSet<usize> = query_property(index, p).iter().map(|wf| wf.doc_id).collect();

        doc_ids = ids;
    }

    let mut docs_ids_facets = HashSet::new();

    if let Some(facets) = &query.facets {


        // lookup facet properties and return documents which has these facets un the desired properties




        // intersect facet doc ids and properties doc ids
        // TODO: Make sure we call intersect on the smaller of the sets as the left
        // see https://stackoverflow.com/questions/35439376/why-is-python-set-intersection-faster-than-rust-hashset-intersection

        // TODO: maybe have docs_ids as vec and just run through them and take the X first resulst that exists in the facet doc_ids??

        doc_ids = docs_ids_facets.intersection(&doc_ids).copied().collect();
    };



    SearchResultIds {
        doc_ids: doc_ids

    }

}

fn query_property(index: &SearchableIndex, query: &PropertyQuery) -> Vec::<WordFrequency> {

    return match &query.query_data {
        PropertyType::Text(query_text) => query_text_property(index, &query.name, query_text),
        _ => panic!("Not implemented query on other than Text")
    }
}


fn query_text_property(index: &SearchableIndex, property: &str, query_text: &str) -> Vec::<WordFrequency> {


    let map = index.get_property_map_text(property);

    let mut res = Vec::new();
    if let Some(v) = map.get(query_text) {

        res = v.clone();
    }

    res


}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::index::*;
    use crate::query::tests::searchable_index::*;
    use crate::query::tests::index_types::*;


    fn create_index() -> SearchableIndex {

        let documents = vec! [
            ProcessedDocument::new (
                vec! [
                    IndexProperty {
                        name: "content".to_string(),
                        data: PropertyType::Text("lorup ipsum content for you needs. With lorup repeats".to_string())
                    },

                    IndexProperty {
                        name: "count".to_string(),
                        data: PropertyType::Integer(10)
                    }
                ],
            )

        ];

        SearchableIndex::from_documents(&documents)

    }


    #[test]
    fn get_docs_0() {

        let index = create_index();


        let properties = vec![
            PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("unknown".to_string())
            }
        ];

        let query = IndexQuery {
            criterion: Criterion {properties: properties.clone()},
            facets: None

        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 0);

    }


    #[test]
    fn get_docs_1() {

        let index = create_index();


        let properties = vec![
            PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            }
        ];

        let query = IndexQuery {
            criterion: Criterion {properties: properties.clone()},
            facets: None
        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 1);
    }


    #[test]
    fn get_docs_facets_0() {

        let index = create_index();


        let properties = vec![
            PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            }
        ];

        let facet_properties = vec![
            PropertyQuery {
                name: "count".to_string(),
                query_data: PropertyType::Integer(1)
            }
        ];



        let facet_query = FacetQuery {
            criterion: Some(Criterion { properties: facet_properties.clone() }),
            sub_queries: Vec::new(),
            query_type: FacetQueryType::And


        };

        let query = IndexQuery {
            criterion: Criterion {properties: properties.clone()},
            facets: None
        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 0);
    }



    #[test]
    fn get_docs_facets_1() {

        let index = create_index();


        let properties = vec![
            PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            }
        ];

        let facet_properties = vec![
            PropertyQuery {
                name: "count".to_string(),
                query_data: PropertyType::Integer(10)
            }
        ];



        let facet_query = FacetQuery {
            criterion: Some(Criterion { properties: facet_properties.clone() }),
            sub_queries: Vec::new(),
            query_type: FacetQueryType::And


        };

        let query = IndexQuery {
            criterion: Criterion {properties: properties.clone()},
            facets: None
        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 1);
    }

}
