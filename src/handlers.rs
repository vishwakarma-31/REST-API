use axum::{extract::{Query, State}, Json};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use serde::Deserialize;

use crate::errors::AppError;
use crate::models::Employee;
use crate::query_builder::{build_employees_query, QueryParams};

#[derive(Deserialize)]
pub struct EmployeeQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    #[serde(flatten)]
    pub filters: HashMap<String, String>,
}

pub async fn list_employees(
    State(pool): State<PgPool>,
    Query(q): Query<EmployeeQuery>,
) -> Result<Json<Vec<Employee>>, AppError> {
    let mut filters = q.filters.clone();
    filters.remove("search");
    filters.remove("sort_by");
    filters.remove("order");

    let params = QueryParams {
        search: q.search,
        filters,
        sort_by: q.sort_by,
        order: q.order,
    };

    let (sql, binds) = build_employees_query(&params)?;
    let mut query = sqlx::query(&sql);
    for b in binds {
        query = query.bind(b);
    }

    let rows = query.fetch_all(&pool).await?;
    let employees = rows.into_iter().map(|r| Employee {
        id: r.get("id"),
        name: r.get("name"),
        department: r.get("department"),
        role: r.get("role"),
        salary: r.get("salary"),
        location: r.get("location"),
        joined_at: r.get("joined_at"),
    }).collect();

    Ok(Json(employees))
}
