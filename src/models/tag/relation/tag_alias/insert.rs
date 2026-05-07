use tracing::debug;

use crate::SqlxError;
use crate::Tag;
use crate::TagAlias;

impl Tag {
    /// Add an alias to this tag.
    ///
    /// This doesn't duplicate aliases, but prevent adding an alias identical to the name
    pub async fn add_alias(
        &self,
        conn: &mut sqlx::SqliteConnection,
        name: &str,
    ) -> Result<Option<TagAlias>, SqlxError> {
        if self.name == name {
            debug!("Ignoring alias addition {name}");
            return Ok(None);
        }

        TagAlias {
            id: 0,
            name: name.to_string(),
            tag_id: self.id,
        }
        .insert(conn)
        .await
        .map(Some)
    }
}
