use itertools::Itertools as _;

use crate::query::trait_entry_filter::EntryFilter;
use crate::tests::fixtures::test_data::get_test_library;

pub async fn assert_eq_entries<T>(query: T, mut expected: Vec<i64>)
where
    T: EntryFilter,
{
    let lib = get_test_library().await;

    println!();
    println!("Query`\n{}", query.as_entry_select(&mut 1).unwrap());
    println!();

    let results = query
        .fetch_all(&mut *lib.db.get().await.unwrap())
        .await
        .unwrap();

    let mut result_ids = results.into_iter().map(|entr| entr.id).collect_vec();
    result_ids.sort();
    expected.sort();

    assert_eq!(result_ids, expected);
}
