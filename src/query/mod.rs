pub mod and;
pub mod entries_with_tags;
pub mod entry_search_query;
pub mod eq_absolute_path;
pub mod eq_any_entry_id;
pub mod eq_any_tag_id;
pub mod eq_entry_field;
pub mod eq_entry_id;
pub mod eq_entry_name;
pub mod eq_folder;
pub mod eq_tag_id;
pub mod eq_tag_or_children;
pub mod eq_tag_string;
pub mod not;
pub mod or;
pub mod parsing;
pub mod tag_search_query;
pub mod trait_entry_filter;
pub mod trait_tag_filter;

use crate::query::parsing::expression::parse_expression;
use sqlx::Sqlite;
use sqlx::query::QueryAs;
use sqlx::sqlite::SqliteArguments;

pub type SQLQuery<'q, O> = QueryAs<'q, Sqlite, O, SqliteArguments<'q>>;
