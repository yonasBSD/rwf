use super::{FromRow, Model};

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Row {
    row: Arc<tokio_postgres::Row>,
}

impl Model for Row {
    fn table_name() -> String {
        unimplemented!()
    }

    fn primary_key() -> String {
        unimplemented!()
    }

    fn foreign_key() -> String {
        unimplemented!()
    }
}

impl FromRow for Row {
    fn from_row(row: tokio_postgres::Row) -> Self {
        Self::new(row)
    }
}

impl std::ops::Deref for Row {
    type Target = tokio_postgres::Row;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

impl Row {
    pub fn new(row: tokio_postgres::Row) -> Self {
        Self { row: Arc::new(row) }
    }
}

#[cfg(test)]
mod test {
    use super::super::{Query, ToSql};
    use super::*;

    #[test]
    fn test_random_query() {
        let query = Row::find_by_sql("SELECT 1");
        assert_eq!(query.to_sql(), "SELECT 1");

        let query = Query::<Row>::select("users");
        assert_eq!(query.to_sql(), "SELECT * FROM \"users\"");
    }
}
