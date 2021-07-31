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
    first_property: PropertyQuery,
    additional_properties: Option<Vec::<PropertyQuery>>,
    operation: Operation
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    And,
    Or
}

impl Operation {

    pub fn combine(&self, set1: &HashSet<DocId>, set2: &HashSet<DocId>) -> HashSet<DocId> {

        let smaller;
        let bigger;

        if set1.len() < set2.len() {
            smaller = &set1;
            bigger = &set2;
        } else {
            smaller = &set2;
            bigger = &set1;

        }

        return match self {
            And => smaller.intersection(bigger).copied().collect(),
            Or => smaller.union(bigger).copied().collect()
        }

    }
}

pub struct FacetQuery {
    criterion: Option<Criterion>,
    sub_queries: Vec::<FacetQuery>,
    operation: Operation,
}



pub fn query_index(index: &SearchableIndex, query: &IndexQuery) -> SearchResultIds {

    // Gather freq of each property query

    let mut doc_ids = query_criterion(index, &query.criterion);

    if let Some(facets) = &query.facets {

        let mut doc_ids_facets = HashSet::new();

        // lookup facet properties and return documents which has these facets un the desired properties

        if let Some(criterion) = &facets.criterion {
            doc_ids_facets = query_criterion(index, criterion);
        }

        // intersect facet doc ids and properties doc ids
        // TODO: Make sure we call intersect on the smaller of the sets as the left
        // see https://stackoverflow.com/questions/35439376/why-is-python-set-intersection-faster-than-rust-hashset-intersection

        // TODO: maybe have doc_ids as vec and just run through them and take the X first resulst that exists in the facet doc_ids??

        println!("FacetIds {:?}",doc_ids_facets.len());

        doc_ids = facets.operation.combine(&doc_ids, &doc_ids_facets);
    };

    SearchResultIds {
        doc_ids: doc_ids

    }
}

fn query_criterion(index: &SearchableIndex, criterion: &Criterion) -> HashSet::<DocId> {

    let mut doc_ids: HashSet<DocId> = query_property(index, &criterion.first_property).iter().map(|wf| wf.doc_id).collect();

    if let Some(properties) = &criterion.additional_properties {
        for p in properties {

            let ids: HashSet<DocId> = query_property(index, p).iter().map(|wf| wf.doc_id).collect();

            doc_ids = criterion.operation.combine(&doc_ids, &ids)
        }
    }

    doc_ids

}



fn query_property(index: &SearchableIndex, query: &PropertyQuery) -> Vec::<WordFrequency> {

    return match &query.query_data {
        PropertyType::Text(query_text) => query_text_property(index, &query.name, query_text),
        PropertyType::Integer(val) => query_integer_property(index, &query.name, val),
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


fn query_integer_property(index: &SearchableIndex, property: &str, val: &IntegerT) -> Vec::<WordFrequency> {
    let map = index.get_property_map_integer(property);

    let mut res = Vec::new();
    if let Some(v) = map.get(val) {

        res = v.clone();
    }

    res
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::index::*;
    use crate::query::tests::searchable_index::*;



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

        let criterion = Criterion {
            first_property: PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("unknown".to_string())
            },
            additional_properties: None,
            operation: Operation::Or
        };



        let query = IndexQuery {
            criterion,
            facets: None
        };



        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 0);

    }


    #[test]
    fn get_docs_1() {

        let index = create_index();

        let criterion = Criterion {
            first_property: PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            },
            additional_properties: None,
            operation: Operation::Or
        };



        let query = IndexQuery {
            criterion,
            facets: None
        };

        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 1);
    }


    #[test]
    fn get_docs_facets_0() {

        let index = create_index();


        let criterion = Criterion {
            first_property: PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            },
            additional_properties: None,
            operation: Operation::Or
        };

        let facet_criterion = Criterion {
            first_property: PropertyQuery {
                name: "count".to_string(),
                query_data: PropertyType::Integer(1)
            },
            additional_properties: None,
            operation: Operation::Or
        };

        let facet = FacetQuery {
            criterion: Some(facet_criterion),
            sub_queries: Vec::new(),
            operation: Operation::And
        };


        let query = IndexQuery {
            criterion,
            facets: Some(facet)
        };

        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 0);
    }



    #[test]
    fn get_docs_facets_1() {

        let index = create_index();


        let criterion = Criterion {
            first_property: PropertyQuery {
                name: "content".to_string(),
                query_data: PropertyType::Text("lorup".to_string())
            },
            additional_properties: None,
            operation: Operation::Or
        };


        let facet_criterion = Criterion {
            first_property: PropertyQuery {
                name: "count".to_string(),
                query_data: PropertyType::Integer(10)
            },
            additional_properties: None,
            operation: Operation::Or
        };

        let facet = FacetQuery {
            criterion: Some(facet_criterion),
            sub_queries: Vec::new(),
            operation: Operation::And
        };



        let query = IndexQuery {
            criterion,
            facets: Some(facet)
        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 1);
    }

}
