use crate::Library;
use crate::Tag;

pub(super) async fn add_test_tag_parents(lib: &Library) {
    add_tag_parent(lib, "Cat", "Maxwell").await;
    add_tag_parent(lib, "Cat", "OIIA").await;
    add_tag_parent(lib, "Meme", "Maxwell").await;
    add_tag_parent(lib, "Meme", "Doge").await;
    add_tag_parent(lib, "Meme", "OIIA").await;
    add_tag_parent(lib, "Dog", "Doge").await;
}

async fn add_tag_parent(lib: &Library, parent: &str, child: &str) {
    let conn = &mut *lib.db.get().await.unwrap();

    let parent = Tag::find_by_exact_name(conn, parent)
        .await
        .unwrap()
        .first()
        .cloned()
        .unwrap();

    let child = Tag::find_by_exact_name(conn, child)
        .await
        .unwrap()
        .first()
        .cloned()
        .unwrap();

    parent.add_child(conn, child.id).await.unwrap();
}
