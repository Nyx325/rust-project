use rusqlite::Row;

use crate::data::repo::client_repo::SearchCriteria;

#[allow(unused)]
pub trait Repository<Model, Err>
where
    Model: PartialEq + Clone + serde::Serialize,
{
    fn add(&mut self, item: &Model) -> Result<(), Err>;
    fn drop(&mut self, item: &mut Model) -> Result<(), Err>;
    fn delete(&mut self, item: &Model) -> Result<(), Err>;
    fn modify(&mut self, item: &Model) -> Result<(), Err>;
}

#[allow(unused)]
pub trait Manager<Model, SearchCriteria, Err>
where
    Model: PartialEq + Clone,
{
    fn valid_item(&self, item: &Model) -> Result<(), Err>;
    fn last_search(&self) -> Option<LastSearch<SearchCriteria>>;
    fn set_last_search(&mut self, search: LastSearch<SearchCriteria>);
    fn last_selected(&self) -> Option<Model>;
    fn set_last_selected(&mut self, item: Model);
}

#[allow(unused)]
pub trait Finder<Model, SearchCriteria, Err>
where
    Model: PartialEq + Clone,
{
    fn from_row(row: &Row) -> Result<Model, Err>;
    fn page_size(&self) -> u128;
    fn search_by_id(&self, id: u32) -> Result<Option<Model>, Err>;
    fn search_by(&self, criteria: &SearchCriteria, page_number: u128) -> Result<String, Err>;
}

#[allow(unused)]
#[derive(Debug)]
pub struct LastSearch<SearchCriteria> {
    pub page: u128,
    pub criteria: SearchCriteria,
    pub result: String,
}

#[allow(unused)]
impl LastSearch<SearchCriteria> {
    pub fn new(page: u128, criteria: SearchCriteria, result: String) -> LastSearch<SearchCriteria> {
        Self {
            page,
            criteria,
            result,
        }
    }
}
