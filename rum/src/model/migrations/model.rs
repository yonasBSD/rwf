use crate::model::{FromRow, Model, ToValue, Value};
use time::OffsetDateTime;

use std::path::PathBuf;

use super::Direction;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Migration {
    id: Option<i64>,
    pub version: i64,
    pub name: String,
    pub applied_at: Option<OffsetDateTime>,
}

impl FromRow for Migration {
    fn from_row(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            version: row.get("version"),
            name: row.get("name"),
            applied_at: row.get("applied_at"),
        }
    }
}

impl Model for Migration {
    fn primary_key() -> String {
        "id".to_string()
    }

    fn table_name() -> String {
        "rum_migrations".to_string()
    }

    fn foreign_key() -> String {
        "rum_migration_id".to_string()
    }

    fn id(&self) -> Value {
        self.id.to_value()
    }

    fn values(&self) -> Vec<Value> {
        vec![
            self.version.to_value(),
            self.name.to_value(),
            self.applied_at.to_value(),
        ]
    }

    fn column_names() -> Vec<String> {
        vec![
            "version".to_string(),
            "name".to_string(),
            "applied_at".to_string(),
        ]
    }
}

impl Migration {
    pub(crate) fn path(&self, direction: Direction) -> PathBuf {
        PathBuf::from(format!(
            "{}_{}.{}.sql",
            self.version,
            self.name,
            match direction {
                Direction::Up => "up",
                Direction::Down => "down",
            }
        ))
    }
}