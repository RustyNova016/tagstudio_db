pub struct EqAnyTagId {
    id: Vec<i64>,
}

impl EqAnyTagId {
    pub fn new1(value: i64) -> Self {
        Self { id: vec![value] }
    }
}
