use super::{Escape, ToSql};

/// PostgreSQL table column.
#[derive(Debug, Clone)]
pub struct Column {
    table_name: String,
    column_name: String,
}

impl ToSql for Column {
    fn to_sql(&self) -> String {
        if self.table_name.is_empty() {
            format!(r#""{}""#, self.column_name.escape())
        } else {
            format!(
                r#""{}"."{}""#,
                self.table_name.escape(),
                self.column_name.escape()
            )
        }
    }
}

impl Column {
    /// Create new table column, given the table name and column name.
    ///
    /// Columns are ideally always fully qualified with the table name
    /// to avoid ambiguous errors.
    pub fn new(table_name: impl ToString, column_name: impl ToString) -> Self {
        Self {
            table_name: table_name.to_string(),
            column_name: column_name.to_string(),
        }
    }

    /// Create new table column, given the column name.
    ///
    /// Not fully qualified, so use with care, or you'll get
    /// ambiguous column error when joining, especially with common column
    /// names like "id".
    pub fn name(column_name: impl ToString) -> Self {
        Self::new("", column_name)
    }

    pub fn qualified(&self) -> bool {
        !self.table_name.is_empty()
    }

    pub fn qualify(mut self, table_name: impl ToString) -> Self {
        self.table_name = table_name.to_string();
        self
    }
}

#[derive(Debug, Default)]
pub struct Columns {
    columns: Vec<Column>,
    table_name: Option<String>,
}

impl Columns {
    pub fn table_name(mut self, table_name: impl ToString) -> Self {
        self.table_name = Some(table_name.to_string());
        self
    }
}

impl ToSql for Columns {
    fn to_sql(&self) -> String {
        if self.columns.is_empty() {
            if let Some(ref table_name) = self.table_name {
                format!(r#""{}".*"#, table_name)
            } else {
                "*".to_string()
            }
        } else {
            self.columns
                .iter()
                .map(|column| column.to_sql())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

pub trait IntoColumn {
    fn into_column(&self) -> Column;
}

impl IntoColumn for String {
    fn into_column(&self) -> Column {
        Column::name(self)
    }
}
