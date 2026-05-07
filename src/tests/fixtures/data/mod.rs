use crate::Folder;
use crate::Library;
use crate::tests::fixtures::data::entries::add_test_entries;
use crate::tests::fixtures::data::tag_entries::add_test_tag_entries;
use crate::tests::fixtures::data::tag_parents::add_test_tag_parents;
use crate::tests::fixtures::data::tags::add_test_tags;
use crate::tests::fixtures::raw_library::get_empty_library;

pub mod entries;
pub mod tag_entries;
pub mod tag_parents;
pub mod tags;

/// Return an inmemmory database with testing data
pub async fn get_test_library() -> Library {
    let lib = get_empty_library().await;

    Folder {
        id: 0,
        path: "/tmp".to_string(),
        uuid: "uuid".to_string(),
    }
    .insert(&mut *lib.db.get().await.unwrap())
    .await
    .unwrap();

    add_test_entries(&lib).await;
    add_test_tags(&lib).await;
    add_test_tag_parents(&lib).await;
    add_test_tag_entries(&lib).await;

    lib
}
