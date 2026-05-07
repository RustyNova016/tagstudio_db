pub mod update;

pub struct BooleanField {
    pub value: Option<bool>,
    pub id: i64,
    pub type_key: String,
    pub entry_id: i64,
    pub position: i64,
}
