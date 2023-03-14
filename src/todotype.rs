use chrono::prelude::*;
use yew::{prelude::*, html::onclick};
use uuid::Uuid;
use web_sys::HtmlInputElement;

#[derive(Clone,PartialEq)]
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