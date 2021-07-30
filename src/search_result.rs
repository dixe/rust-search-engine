use std::collections::HashSet as HashSet;

use crate::index::index_types::{DocId};

pub struct SearchResultIds {
    pub doc_ids: HashSet::<DocId>,
}
