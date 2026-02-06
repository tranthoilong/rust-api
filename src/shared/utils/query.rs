use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

// Re-export DTOs từ application để không phá vỡ các import hiện có
pub use crate::application::common::list_params::{ListParams, PaginatedResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "asc" => SortDirection::Asc,
            _ => SortDirection::Desc,
        }
    }

    pub fn as_sql(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Timestamp,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldInfo {
    pub name: &'static str,
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub enum BindValue {
    Text(String),
    Timestamp(NaiveDateTime),
    Uuid(Uuid),
    I64(i64),
}

#[derive(Debug, Clone)]
pub struct BuiltQuery {
    pub sql: String,
    pub binds: Vec<BindValue>,
    pub limit: i64,
    pub sort_field: &'static str,
    #[allow(dead_code)]
    pub sort_dir: SortDirection,
}

#[derive(Debug, Clone)]
pub struct CursorData {
    pub sort_raw: String,
    pub id: Uuid,
}

fn decode_cursor(cursor: &str) -> Result<CursorData, String> {
    let decoded = general_purpose::STANDARD
        .decode(cursor)
        .map_err(|e| format!("Invalid cursor: {e}"))?;
    let decoded_str =
        String::from_utf8(decoded).map_err(|e| format!("Invalid cursor utf8: {e}"))?;
    let mut parts = decoded_str.splitn(2, '|');
    let sort_raw = parts
        .next()
        .ok_or_else(|| "Cursor missing sort value".to_string())?
        .to_string();
    let id_str = parts
        .next()
        .ok_or_else(|| "Cursor missing id value".to_string())?;
    let id = Uuid::parse_str(id_str).map_err(|e| format!("Invalid cursor id: {e}"))?;
    Ok(CursorData { sort_raw, id })
}

pub fn encode_cursor_text(value: &str, id: Uuid) -> String {
    let raw = format!("{value}|{id}");
    general_purpose::STANDARD.encode(raw.as_bytes())
}

pub fn encode_cursor_ts(value: NaiveDateTime, id: Uuid) -> String {
    let dt: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(value, Utc);
    let raw = format!("{}|{id}", dt.to_rfc3339());
    general_purpose::STANDARD.encode(raw.as_bytes())
}

fn sanitize_fields<'a>(
    requested: Option<&str>,
    allowed: &'a [FieldInfo],
    default: &'a [&str],
) -> Vec<&'a FieldInfo> {
    let mut result = Vec::new();
    if let Some(req) = requested {
        for name in req.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let Some(f) = allowed.iter().find(|f| f.name.eq_ignore_ascii_case(name)) {
                result.push(f);
            }
        }
    }
    if result.is_empty() {
        for name in default {
            if let Some(f) = allowed.iter().find(|f| f.name == *name) {
                result.push(f);
            }
        }
    }
    result
}

pub fn build_query(
    base_sql: &str,
    params: &ListParams,
    allowed_fields: &[FieldInfo],
    default_sort: &'static str,
    default_dir: SortDirection,
    default_search_fields: &[&str],
) -> Result<BuiltQuery, String> {
    build_query_with_seed(
        base_sql,
        params,
        allowed_fields,
        default_sort,
        default_dir,
        default_search_fields,
        &[],
        0,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn build_query_with_seed(
    base_sql: &str,
    params: &ListParams,
    allowed_fields: &[FieldInfo],
    default_sort: &'static str,
    default_dir: SortDirection,
    default_search_fields: &[&str],
    base_filters: &[(&str, BindValue)],
    start_index: i32,
) -> Result<BuiltQuery, String> {
    let mut binds: Vec<BindValue> = base_filters.iter().map(|(_, b)| b.clone()).collect();
    let mut clauses: Vec<String> = base_filters.iter().map(|(c, _)| c.to_string()).collect();
    let mut idx: i32 = start_index;

    // search fields
    let search_fields = sanitize_fields(
        params.fields.as_deref(),
        allowed_fields,
        default_search_fields,
    );
    if let Some(search) = params.search.as_ref().filter(|s| !s.is_empty()) {
        let pattern = format!("%{search}%");
        let mut or_parts = Vec::new();
        for field in search_fields.iter() {
            if field.field_type == FieldType::Text {
                idx += 1;
                or_parts.push(format!("{} ILIKE ${}", field.name, idx));
                binds.push(BindValue::Text(pattern.clone()));
            }
        }
        if !or_parts.is_empty() {
            clauses.push(format!("({})", or_parts.join(" OR ")));
        }
    }

    // sort parse
    let (sort_field, sort_dir) = if let Some(sort_raw) = params.sort_by.as_ref() {
        let mut parts = sort_raw.splitn(2, ':');
        let field = parts.next().unwrap_or(default_sort);
        let dir = parts.next().unwrap_or_else(|| default_dir.as_sql());
        let dir = SortDirection::from_str(dir);
        if let Some(info) = allowed_fields
            .iter()
            .find(|f| f.name.eq_ignore_ascii_case(field))
        {
            (info, dir)
        } else {
            (
                allowed_fields
                    .iter()
                    .find(|f| f.name == default_sort)
                    .ok_or_else(|| "default sort field not in allowed_fields".to_string())?,
                default_dir,
            )
        }
    } else {
        (
            allowed_fields
                .iter()
                .find(|f| f.name == default_sort)
                .ok_or_else(|| "default sort field not in allowed_fields".to_string())?,
            default_dir,
        )
    };

    // cursor (bỏ qua nếu chuỗi rỗng)
    if let Some(cursor) = params.cursor.as_ref().filter(|s| !s.is_empty()) {
        let decoded = decode_cursor(cursor)?;
        match sort_field.field_type {
            FieldType::Text => {
                idx += 1;
                let idx_sort = idx;
                idx += 1;
                let idx_id = idx;
                let cmp = match sort_dir {
                    SortDirection::Asc => ">",
                    SortDirection::Desc => "<",
                };
                clauses.push(format!(
                    "({sort_col}, id) {cmp} (${}, ${})",
                    idx_sort,
                    idx_id,
                    sort_col = sort_field.name
                ));
                binds.push(BindValue::Text(decoded.sort_raw));
                binds.push(BindValue::Uuid(decoded.id));
            }
            FieldType::Timestamp => {
                let dt = DateTime::parse_from_rfc3339(&decoded.sort_raw)
                    .map_err(|e| format!("Invalid cursor datetime: {e}"))?
                    .naive_utc();
                idx += 1;
                let idx_sort = idx;
                idx += 1;
                let idx_id = idx;
                let cmp = match sort_dir {
                    SortDirection::Asc => ">",
                    SortDirection::Desc => "<",
                };
                clauses.push(format!(
                    "({sort_col}, id) {cmp} (${}, ${})",
                    idx_sort,
                    idx_id,
                    sort_col = sort_field.name
                ));
                binds.push(BindValue::Timestamp(dt));
                binds.push(BindValue::Uuid(decoded.id));
            }
        }
    }

    let where_sql = if clauses.is_empty() {
        String::new()
    } else {
        let has_where = base_sql.to_ascii_uppercase().contains("WHERE");
        let prefix = if has_where { " AND " } else { " WHERE " };
        format!("{prefix}{}", clauses.join(" AND "))
    };

    let order_sql = format!(
        " ORDER BY {} {}, id {}",
        sort_field.name,
        sort_dir.as_sql(),
        sort_dir.as_sql()
    );

    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    idx += 1;
    let limit_idx = idx;
    let sql = format!("{base_sql}{where_sql}{order_sql} LIMIT ${limit_idx}");
    binds.push(BindValue::I64(limit));

    Ok(BuiltQuery {
        sql,
        binds,
        limit,
        sort_field: sort_field.name,
        sort_dir,
    })
}
