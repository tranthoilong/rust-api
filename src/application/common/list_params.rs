use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListParams {
    pub search: Option<String>,
    pub fields: Option<String>,
    pub sort_by: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
    pub limit: i64,
}

