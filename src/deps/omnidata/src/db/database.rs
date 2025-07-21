

pub trait DatabaseComing {
    fn select(&self);
    fn insert(&self);
    fn update(&self);
    fn delete(&self);
    fn purge(&self);
    fn create_table(&self);
    fn drop_table(&self);
}


