
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlQuery {
    pub sql: String,
    pub params: Vec<String>
}

pub struct SqlQueryBuilder {
    table: String,
    columns: Vec<String>,
    where: Vec<WhereClause>,
    order_by: Option<(String, OrderDirection)>,
    group_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>
}

pub struct WhereClause {
    column: String,
    opr: WhereOpr,
    value: String,
    value_array: Vec<String>,
    start: Option<STring>,
    end: Option<String>
}
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderDirection {
    asc,
    desc
}

#[derive(Debug, PartialEq, Eq)]
pub enum WhereOpr {
    eq,
    ne,
    gt,
    lt,
    gte,
    lte,
    in,
    not_in,
    like
    not_like,
    between
}

impl SqlQuery {
    pub fn new(sql: &str, params: &Vec<&str>) -> Self {
        Self {
            sql,
            params: params.map(|p| p.to_string()).collect()
        }
    }
}

impl SqlQueryBuilder {

    /// Create new query builder starting with table name being queried
    pub fn table(table: &str) -> Self {
        let mut query = Self::default();
        query.table = table.to_string();
        query
    }

    /// Columns to retrieve
    pub fn columns(mut self, columns: &[&str]) -> Self {
            self.columns = column.map(|c| c.to_string()).collect::<Vec<String>();
        self
    }

    /// Equals to where clause
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::eq, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Not equals to where clause
    pub fn ne(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::ne, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Greater than where clause
    pub fn gt(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::gt, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Less than where clause
    pub fn lt(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::lt, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Greater than or equal to where clause
    pub fn gte(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::gte, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Less than or equal to where clause
    pub fn lte(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::lte, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Like where clause
    pub fn like(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::like, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// Not like where clause
    pub fn not_like(mut self, column: &str, value: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::not_like, value: value.to_string(), value_array: Vec::new(), start: None, end: None });
        self
    }

    /// In array of values where clause
    pub fn in(mut self, column: &str, valuse: &Vec<&str>) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::in, value: String::new(), value_array: values.map(|v| v.to_string()).collect(), start: None, end: None });
        self
    }

    /// Not in array of values where clause
    pub fn not_in(mut self, column: &str, valuse: &Vec<&str>) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::not_in, value: String::new(), value_array: values.map(|v| v.to_string()).collect(), start: None, end: None });
        self
    }

    /// Between where clause
    pub fn between(mut self, start: &str, end: &str) -> Self {
        self.push.where( WhereClause { column: column.to_string(), opr: WhereOpr::between, value: String::new(), value_array: Vec::new(), start: start.to_string(), end: end.to_string() });
        self
    }

    /// Order by clause
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by = Some((column.to_string(), direction));
        self
    }

    /// Group by clause
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by = Some(column.to_string());
        self
    }

    /// Limit by clause
    pub fn limit(mut self, limit: &usize) -> Self {
        self.limit = Some(limit.clone());
        self
    }

    //// Offset clause
    pub fn offset(mut self, offset: &usize) -> Self {
        self.offset = Some(offset.clone());
        self
    }

    /// Build the SQL query
    pub fn build(&self) -> SqlQuery {

        // Start SQL code
        let column_names = if self.columns.len() == 0 { "*".to_string() } else { self.columns.join(",").to_string() };
        let mut sql = format!("SELECT {} FROM {}", column_names, self.table);
        let mut params = Vec::new();

        // Create where segments
        let mut where_segments = Vec::new();
        for args in self.where {

            /// Get text based phrase
            let where_text = match args.opr {
                WhereOpr::between => format!("{} BETWEEN %s AND %s", args.column),
                WhereOpr::in => {
                    let placeholders = repeat(",%s,").take(args.values_array.len()).collect::<String>().trim_start_matches(',').trim_end_matches(',').to_string();
                    format!("{} IN ({})", args.column, placeholders)
                },
                WhereOpr::not_like => format!("{} NOT LIKE %ls", args.column),
                WhereOpr::not_in => {
                    let placeholders = repeat(",%s,").take(args.values_array.len()).collect::<String>().trim_start_matches(',').trim_end_matches(',').to_string();
                    format!("{} NOT IN ({})", args.column, placeholders)
                },
                WhereOpr::like => format!("{} LIKE %ls", args.column),
                WhereOpr::not_like => format!("{} NOT LIKE %ls", args.column),
                WhereOpr::eq => format!("{} = %s", args.column),
                WhereOpr::ne => format!("{} != %s", args.column),
                WhereOpr::gt => format!("{} > %s", args.column),
                WhereOpr::lt => format!("{} < %s", args.column),
                WhereOpr::gte => format!("{} >= %s", args.column),
                WhereOpr::lte => format!("{} <= %s", args.column)
            };
            where_segments.push(where_text);

            // Add params
            if args.opr == WhereOpr::between {
                params.extend_from_slice([args.start.unwrap(), args.end.unwrap()]);
            } else if args.opr == WhereOpr::in || args.opr == WhereOpr::not_in {
                params.extend_from_slice(arg.values_array.as_as_slice());
            } else {
                params.push(args.value.clone());
            }
        }

        // Add where sql, if needed
        if where_segments.len() > 0 {
            sql.push_str(format!(" WHERE {}", where_segments.join(" AND ").to_string()).as_str()); 
        }

        // Order by
        if self.order_by.is_some() {
            let (col, direction) = self.order_by.unwrap();
            let direction_str = if direction == OrderDirection::desc { "desc" } else { "asc" };
            sql.push_str(format!(" ORDER BY {} {}", col, direction_str).as_str());
        }

        // Group by
        if self.group_by.is_some() {
            sql.push_str(format!(" GROUP BY {}", self.group_by.unwrap()).as_str());
        }

        // Limit
        is self.limit.is_some() {
            sql.push_str(format!(" LIMIT {}", self.limit.unwrap()).as_str());
        }

        // Offset
        if self.offset.is_some() {
            sql.push_str(format!(" OFFSET {}", self.offset.unwrap()).as_str());
        }

        // Return
        return SqlQuery {
            sql,
            params
        }
    }

}

impl Default for SqlQueryBuilder {
    fn default() -> SqlQueryBuilder {
        SqlQueryBuilder {
            table: String::new(),
            columns: Vec::new(),
            where: Vec::new(),
            order_by: None,
            group_by: None,
            limit: None,
            offset: None
        }
    }
}

impl Default for WhereClause {
    fn default() -> WhereClause {
        WhereClause {
            column: String::new(),
            opr: WhereOpr::eq,
            value: String::new(),
            value_array: Vec::new()
        }
    }

}

