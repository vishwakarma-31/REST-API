use crate::errors::AppError;
use std::collections::HashMap;

pub const ALLOWED_COLUMNS: &[&str] = &["id", "name", "department", "role", "salary", "location", "joined_at"];

pub struct QueryParams {
    pub search: Option<String>,
    pub filters: HashMap<String, String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

pub fn build_employees_query(params: &QueryParams) -> Result<(String, Vec<String>), AppError> {
    let mut sql = String::from("SELECT * FROM employees WHERE 1=1");
    let mut binds = Vec::new();
    let mut idx = 1;

    if let Some(ref s) = params.search {
        let pattern = format!("%{}%", s);
        sql.push_str(&format!(" AND (name ILIKE ${} OR department ILIKE ${} OR role ILIKE ${} OR location ILIKE ${})", idx, idx, idx, idx));
        binds.push(pattern);
        idx += 1;
    }

    for (col, val) in &params.filters {
        if !ALLOWED_COLUMNS.contains(&col.as_str()) {
            return Err(AppError::InvalidFilterColumn);
        }
        sql.push_str(&format!(" AND {} = ${}", col, idx));
        binds.push(val.clone());
        idx += 1;
    }

    if let Some(ref sort_by) = params.sort_by {
        if !ALLOWED_COLUMNS.contains(&sort_by.as_str()) {
            return Err(AppError::InvalidSortColumn);
        }
        let order = params.order.as_deref().unwrap_or("asc").to_uppercase();
        if order != "ASC" && order != "DESC" {
            return Err(AppError::InvalidSortOrder);
        }
        sql.push_str(&format!(" ORDER BY {} {}", sort_by, order));
    }

    Ok((sql, binds))
}
