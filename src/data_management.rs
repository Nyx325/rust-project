use std::{any::Any, collections::HashMap};

#[allow(unused)]
pub trait Repository<T, E>
where
    T: PartialEq + Clone,
{
    fn add(&mut self, item: &T) -> Result<(), E>;
    fn drop(&mut self, item: &mut T) -> Result<(), E>;
    fn delete(&mut self, item: &T) -> Result<(), E>;
    fn modify(&mut self, item: &T) -> Result<(), E>;
    fn search_by_id(&self, id: u32) -> Result<Option<T>, E>;
    fn search_by_attributes(
        &mut self,
        page: usize,
        hash_map: HashMap<String, Box<dyn Any>>,
    ) -> Result<String, E>;
}

#[allow(unused)]
pub trait Manager<T, E>
where
    T: PartialEq + Clone,
{
    fn valid_item(&self, item: &T) -> Result<(), E>;
    fn last_search(&self) -> Option<LastSearch>;
    fn set_last_search(&mut self, search: LastSearch);
    fn last_selected(&self) -> Option<T>;
    fn set_last_selected(&mut self, item: T);
}

#[derive(Debug)]
pub struct LastSearch {
    pub page: usize,
    pub hashmap: HashMap<String, Box<dyn Any>>,
    pub result: String,
}
