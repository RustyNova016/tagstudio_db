use core::ops::AddAssign as _;

use crate::query::SQLQuery;
use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::trait_entry_filter::EntryFilter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EqEntryField {
    pub field_type: String,
    pub value: FieldValue,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldValue {
    Boolean(bool),
    Datetime(chrono::NaiveDate),
    Text(String),
}

impl EntryFilter for EqEntryField {
    fn get_where_condition(&self, bind_id: &mut u64) -> Option<String> {
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

    fn bind<'q, O>(&'q self, query: SQLQuery<'q, O>) -> SQLQuery<'q, O> {
        let query = query.bind(self.field_type.to_string());

        match &self.value {
            FieldValue::Boolean(val) => query.bind(val),
            FieldValue::Datetime(val) => query.bind(val),
            FieldValue::Text(val) => query.bind(val),
        }
    }
}

impl From<EqEntryField> for EntrySearchQuery {
    fn from(value: EqEntryField) -> Self {
        EntrySearchQuery::EqEntryField(value)
    }
}

#[cfg(test)]
pub mod test {
    use crate::query::eq_entry_field::EqEntryField;
    use crate::query::eq_entry_field::FieldValue;
    use crate::tests::fixtures::assertions::assert_eq_entries;

    #[tokio::test]
    pub async fn eq_entry_id_test() {
        assert_eq_entries(
            EqEntryField {
                field_type: "DESCRIPTION".into(),
                value: FieldValue::Text("A very dingus cat".to_string()),
            },
            vec![4],
        )
        .await;
    }
}
