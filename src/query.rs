use crate::index::index_types::*;
use crate::index::searchable_index::{SearchableIndex};
use crate::search_result::{SearchResultIds};



// This is a processed query where a search query and maybe some facets are
// procesed to produce this query that the index querier can execute on an index


pub struct IndexQuery {
    properties: Vec::<PropertyQuery>,
}

#[derive(Clone)]
pub struct PropertyQuery {
    name: PropertyName,
    query_data: PropertyType,
}




pub fn query_index(index: &SearchableIndex, query: &IndexQuery) -> SearchResultIds {

    // Gather freq of each property query




    let mut doc_ids = Vec::new();

    for p in &query.properties {

        let ids = query_property(index, p).iter().map(|wf| wf.doc_id).collect();

        doc_ids = ids;

    }

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


    println!("{:#?}", res);
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
            properties: properties.clone()
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
            properties: properties.clone()
        };


        let res = query_index(&index, &query);

        assert_eq!(res.doc_ids.len(), 1);
    }

}
