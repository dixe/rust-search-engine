pub type PropertyName = String;

pub type DocId = usize;
pub type IntegerT = u32;
pub type TextT = String;
pub type SortableTextT = String;


#[derive(Copy, Clone, Debug)]
pub struct WordFrequency {
    pub doc_id: DocId,
    pub frequency: u32,
}

#[derive(Clone)]
pub struct IndexProperty {
    pub name: PropertyName,
    pub data: PropertyType
}


#[derive(Clone)]
pub enum PropertyType {
    Integer(u32),
    Text(String),
    SortableText(String), // enforce some kind of size limit to ensure performant sorting
    //Date Type
}
