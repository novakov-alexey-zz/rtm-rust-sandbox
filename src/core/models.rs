//#[macro_use]
use core::schema::*;
use std::time::SystemTime;

//#[derive(Debug, DbEnum, PartialEq)]
//pub enum PriorityEnum {
//    High,
//    Medium,
//    Low,
//    NoPriority,
//}

#[derive(Identifiable, Queryable, Debug, PartialEq, AsChangeset)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub added: SystemTime,
    pub due: SystemTime,
    pub list: String,
    pub notes: String,
    pub completed: bool,
    pub priority: String,
}