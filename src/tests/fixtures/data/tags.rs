use crate::Library;
use crate::Tag;

pub(super) async fn add_test_tags(lib: &Library) {
    add_tag(lib, "Cat").await;
    add_tag(lib, "Maxwell").await;
    add_tag(lib, "Meme").await;
    add_tag(lib, "Doge").await;
    add_tag(lib, "Dog").await;
    add_tag(lib, "OIIA").await;
}

async fn add_tag(lib: &Library, name: &str) {
    Tag::from(name)
        .insert_tag(&mut *lib.db.get().await.unwrap())
        .await
        .unwrap();
}
