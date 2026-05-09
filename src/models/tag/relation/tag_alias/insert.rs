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

#[cfg(test)]
mod test {
    use crate::Tag;
    use crate::TagAlias;
    use crate::tests::fixtures::data::get_test_library;

    #[tokio::test]
    async fn should_add_alias_to_tag() {
        let lib = get_test_library().await;
        let conn = &mut lib.db.get().await.unwrap();
        let cat_tag = Tag::find_by_exact_name(conn, "Cat")
            .await
            .unwrap()
            .pop()
            .unwrap();

        cat_tag.add_alias(conn, "Meowers").await.unwrap();

        let mut aliases = TagAlias::find_by_name(conn, "Meowers", cat_tag.id)
            .await
            .unwrap();
        assert_eq!(aliases.len(), 1);
        let alias = aliases.pop().unwrap();
        assert_eq!(&alias.name, "Meowers");
    }
}
