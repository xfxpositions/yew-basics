use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone,PartialEq,Debug)]
pub struct Todo{
    pub id:Uuid,
    pub title:String,
    pub created_at: DateTime<Local>,
    pub completed:bool,
}
impl Todo {
    pub fn new(title:String) -> Todo{
        return Todo { id: Uuid::new_v4(), title: title, created_at: Local::now(), completed: false }
    }
}