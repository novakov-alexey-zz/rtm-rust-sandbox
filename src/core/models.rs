//#[macro_use]
use core::schema::*;
use chrono::NaiveDateTime;

//TODO: 2. replace Task.priority String with enum
//#[derive(Debug, DbEnum, PartialEq)]
//pub enum PriorityEnum {
//    High,
//    Medium,
//    Low,
//    NoPriority,
//}

//TODO: 1. replace SystemTime with DateTime from chrono crate
#[derive(Identifiable, Queryable, Debug, PartialEq, Insertable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub added: NaiveDateTime,
    pub due: NaiveDateTime,
    pub list: String,
    pub notes: String,
    pub completed: bool,
    pub priority: String,
}