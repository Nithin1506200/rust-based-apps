use serde::Deserialize;

#[derive(Deserialize,Clone,Debug)]
pub struct CreateEntryData {
    pub title:String,
    pub date:i64,
}


#[derive(Deserialize,Clone)]
pub struct UpdateEntryData {
    pub title:String,

}