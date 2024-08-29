#[allow(unused)]
pub trait Repository<T, E> 
where 
    T: PartialEq + Clone
{
    fn add(&mut self, item: &T) -> Result<(), E>;
    fn drop(&mut self, item: &T) -> Result<(), E>;
    fn delete(&mut self, item: &T) -> Result<(), E>;
    fn modify(&mut self, item: &T) -> Result<(), E>;
    fn search_by_attributes(&mut self, page: usize, json_hashmap: String) 
        -> Result<String,E>;
}

#[allow(unused)]
pub trait Manager<T, E>
where 
    T: PartialEq + Clone
{
    fn repo(&mut self) -> &mut dyn Repository<T, E>;
    fn valid_item(&self, item: &T) -> Result<(), E>;
    fn last_search(&self) -> Option<String>;
    fn set_last_search(&mut self, search: String);
    fn last_selected(&self) -> Option<T>;
    fn set_last_selected(&mut self, item: T);
}


#[macro_export]
macro_rules! impl_repository_for_manager {
    ($manager:ty, $type:ty, $error:ty) => {
        impl Repository<$type, $error> for $manager {
            fn add(&mut self, item: &$type) -> Result<(), $error> {
                // Validar antes de delegar al repositorio
                self.valid_item(item)?;
                self.repo().add(item)
            }

            fn drop(&mut self, item: &$type) -> Result<(), $error> {
                // Validar antes de delegar al repositorio
                self.valid_item(item)?;
                self.repo().drop(item)
            }

            fn delete(&mut self, item: &$type) -> Result<(), $error> {
                self.repo().delete(item)
            }

            fn modify(&mut self, item: &$type) -> Result<(), $error> {
                // Validar antes de delegar al repositorio
                self.valid_item(item)?;
                self.repo().modify(item)
            }

            fn search_by_attributes(
                &mut self,
                page: usize,
                json_hashmap: String,
            ) -> Result<String, $error> {
                // Buscar y almacenar el último resultado de búsqueda
                let result = &mut self.repo().search_by_attributes(page, json_hashmap)?;
                self.set_last_search(result.clone());
                Ok(result.to_string())
            }
        }
    };
}
