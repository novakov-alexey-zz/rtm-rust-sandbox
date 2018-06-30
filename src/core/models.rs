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

//TODO: 1. replace SystemTime with DateTime from chrono crate
#[derive(Identifiable, Queryable, Debug, PartialEq, Insertable)]
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