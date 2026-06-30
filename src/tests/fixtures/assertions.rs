use itertools::Itertools as _;

use crate::query::trait_entry_filter::QueryEntryFilter;
use crate::tests::fixtures::data::get_test_library;

pub async fn assert_eq_entries<T>(query: T, mut expected: Vec<&str>)
where
    T: QueryEntryFilter + Sync,
{
    let lib = get_test_library().await;

    // Run the query

    println!();
    println!("Query`\n{}", query.as_entry_select(&mut 1).unwrap());
    println!();

    let results = query
        .fetch_all(&mut lib.db.get().await.unwrap())
        .await
        .unwrap();

    let mut result_paths = results.into_iter().map(|entr| entr.path).collect_vec();
    result_paths.sort();
    expected.sort();

    // // Fetch the expected results
    // let conn = &mut lib.db.get().await.unwrap();
    // let mut expected_ids = Vec::new();
    // for expected in expected {
    //     let entry = Entry::find_by_path(conn, &expected)
    //         .await
    //         .unwrap()
    //         .first()
    //         .cloned()
    //         .unwrap();

    //     expected_ids.push(entry.id);
    // }

    assert_eq!(result_paths, expected);
}
