use crate::query::Queryfragments;
use crate::query::SQLQuery;

pub struct TagAnd(pub Queryfragments, pub Queryfragments);

impl TagAnd {
    pub fn get_subquery(&self, id: &str) -> String {
        let q_a = self.0.get_subquery(&format!("{id}_0"));
        let q_b = self.1.get_subquery(&format!("{id}_1"));

        if !q_a.is_empty() && !q_b.is_empty() {
            format!("{q_a}, {q_b}")
        } else if q_a.is_empty() && q_b.is_empty() {
            q_a // Prevent realocation of an empty string
        } else if !q_a.is_empty() {
            q_a
        } else {
            q_b
        }
    }

    pub fn get_where_condition(&self, id: &str) -> String {
        format!(
            "({} AND {})",
            self.0.get_where_condition(&format!("{id}_0")),
            self.1.get_where_condition(&format!("{id}_1")),
        )
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        let query = self.0.bind(query);
        self.1.bind(query)
    }
}

impl From<TagAnd> for Queryfragments {
    fn from(value: TagAnd) -> Self {
        Queryfragments::And(Box::new(value))
    }
}
