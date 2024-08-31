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

    fn total_pages(total_registers: u64, page_size: u64) -> u64 {
        if page_size == 0 {
            panic!("Page size must be non-zero");
        }

        // Convertir a f64, realizar la división y redondear
        let result = (total_registers as f64 / page_size as f64).ceil() as u64;
        result
    }
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
    fn page_size(&self) -> u64;
    fn search_by_id(&self, id: u32) -> Result<Option<Model>, Err>;
    fn search_by(
        &mut self,
        criteria: &SearchCriteria,
        page_number: u64,
    ) -> Result<LastSearch<SearchCriteria>, Err>;
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub struct LastSearch<SearchCriteria> {
    pub total_pages: u64,
    pub page: u64,
    pub criteria: SearchCriteria,
    pub result: String,
}

#[allow(unused)]
impl LastSearch<SearchCriteria> {
    pub fn new(
        page: u64,
        total_pages: u64,
        criteria: SearchCriteria,
        result: String,
    ) -> LastSearch<SearchCriteria> {
        Self {
            total_pages,
            page,
            criteria,
            result,
        }
    }
}
