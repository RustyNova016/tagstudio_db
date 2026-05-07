use crate::Entry;
use crate::Library;

pub(super) async fn add_test_entries(lib: &Library) {
    add_entry(lib, "maxwell.png").await;
    add_entry(lib, "doge.png").await;
    add_entry(lib, "doge_and_maxwell.png").await;
    add_entry(lib, "OIIA.png").await;
    add_entry(lib, "somwhere/far/away.png").await;
}

async fn add_entry(lib: &Library, name: &str) {
    Entry {
        date_added: None,
        date_created: None,
        date_modified: None,
        filename: name.split('/').last().unwrap().to_string(),
        folder_id: 0,
        id: 0,
        path: name.to_string(),
        suffix: name.split('.').last().unwrap().to_string(),
    }
    .insert(&mut *lib.db.get().await.unwrap())
    .await
    .unwrap();
}
