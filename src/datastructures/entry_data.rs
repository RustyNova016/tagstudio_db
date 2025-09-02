// use crate::Entry;
// use crate::Tag;

// pub struct EntryData {
//     entry: Entry,
//     tags: Vec<Tag>
// }

// impl EntryData {
//     pub fn from_entry(conn: &mut sqlx::SqliteConnection, entry: Entry) -> Result<Self, crate::Error> {
//         let tags = entry.get_tags(conn)
//     }
// }