use crate::Entry;
use crate::Library;
use crate::Tag;

pub(super) async fn add_test_tag_entries(lib: &Library) {
    add_tag_entry(lib, "maxwell.png", "Maxwell").await;
    add_tag_entry(lib, "doge.png", "Doge").await;
    add_tag_entry(lib, "doge_and_maxwell.png", "Doge").await;
    add_tag_entry(lib, "doge_and_maxwell.png", "Maxwell").await;
    add_tag_entry(lib, "OIIA.png", "OIIA").await;
}

async fn add_tag_entry(lib: &Library, entry: &str, tag: &str) {
    let conn = &mut *lib.db.get().await.unwrap();

    let entry = Entry::find_by_path(conn, entry)
        .await
        .unwrap()
        .first()
        .cloned()
        .unwrap();

    let tag = Tag::find_by_exact_name(conn, tag)
        .await
        .unwrap()
        .first()
        .cloned()
        .unwrap();

    entry.add_tag(conn, &tag).await.unwrap();
}
