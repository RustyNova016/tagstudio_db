use core::ops::AddAssign;

use crate::query::Queryfragments;
use crate::query::SQLQuery;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqField {
    field_type: String,
    value: FieldValue,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldValue {
    Boolean(bool),
    Datetime(chrono::NaiveDate),
    Text(String),
}

impl EqField {
    pub fn new(field_type: String, value: FieldValue) -> Self {
        Self { field_type, value }
    }

    pub fn get_subquery(&self, bind_id: &mut u64) -> Option<String> {
        bind_id.add_assign(2);
        None
    }

    pub fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
        let type_id = *bind_id;
        bind_id.add_assign(1);
        let value_id = *bind_id;
        bind_id.add_assign(1);

        Some(format!(
            "`entries`.`id` IN (
                    SELECT `entry_id` 
                    FROM `boolean_fields`
                    WHERE `type_key` = ${type_id} AND `value` = ${value_id}

                    UNION

                    SELECT `entry_id` 
                    FROM `datetime_fields`
                    WHERE `type_key` = ${type_id} AND `value` = ${value_id}

                    UNION

                    SELECT `entry_id` 
                    FROM `text_fields`
                    WHERE `type_key` = ${type_id} AND `value` = ${value_id}
                )"
        ))
    }

    pub fn bind<'q>(&'q self, query: SQLQuery<'q>) -> SQLQuery<'q> {
        let query = query.bind(self.field_type.to_string());

        match &self.value {
            FieldValue::Boolean(val) => query.bind(val),
            FieldValue::Datetime(val) => query.bind(val),
            FieldValue::Text(val) => query.bind(val),
        }
    }
}

impl From<EqField> for Queryfragments {
    fn from(value: EqField) -> Self {
        Queryfragments::EqField(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::Queryfragments;
    use crate::query::eq_field::EqField;
    use crate::query::eq_field::FieldValue;
    use crate::tests::fixtures::test_data::get_test_library;

    #[tokio::test]
    pub async fn tag_eq_test() {
        let lib = get_test_library().await;

        let result = Queryfragments::from(EqField::new(
            "DESCRIPTION".into(),
            FieldValue::Text("A very dingus cat".to_string()),
        ))
        .fetch_all(&mut lib.db.get().await.unwrap())
        .await
        .unwrap();
        assert_eq!(result.len(), 1);
    }
}
