use crate::Library;
use crate::Tag;

pub(super) async fn add_test_tag_aliases(lib: &Library) {
    add_tag_parent(lib, "Cat", "Kitty").await;
}

async fn add_tag_parent(lib: &Library, tag: &str, alias: &str) {
    let conn = &mut *lib.db.get().await.unwrap();

    let tag = Tag::find_by_exact_name(conn, tag)
        .await
        .unwrap()
        .first()
        .cloned()
        .unwrap();

    tag.add_alias(conn, alias).await.unwrap();
}
