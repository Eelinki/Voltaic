use std::collections::{BTreeMap};
use sqlx::{query_as, Sqlite, SqlitePool, Type};
use sqlx::sqlite::{SqliteRow};
use crate::http::{Error, Result};
use sqlx::Column;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use crate::repository::Value;

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub table_name: String,
}

pub async fn get_collections(pool: &SqlitePool) -> Result<Vec<Collection>> {
    let collections = query_as!(Collection, "SELECT id, name, slug, table_name FROM collections")
        .fetch_all(pool)
        .await?;

    Ok(collections)
}

pub async fn get_all(pool: &SqlitePool, name: String) -> Result<Vec<BTreeMap<String, Value>>> {
    let query = format!(
        r#"
        SELECT *
        FROM {}
        "#, name);

    let mut result = Vec::new();

    let rows = sqlx::query(&query).fetch_all(pool).await?;
    for row in &rows {
        result.push(map_row(row).await?);
    }

    Ok(result)
}

pub async fn get_item(pool: &SqlitePool, name: String, id: i64) -> Result<BTreeMap<String, Value>> {
    let query = format!(
        r#"
        SELECT *
        FROM {}
        WHERE id = {}
        "#, name, id);

    let row = sqlx::query(&query)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::NotFound)?;

    map_row(&row).await
}

async fn map_row(row: &SqliteRow) -> Result<BTreeMap<String, Value>> {
    let mut result = BTreeMap::new();

    let cols = row.columns();
    for _col in cols {
        macro_rules! match_type {
            ($type: ty, $val: expr) => {
                if <$type as Type<Sqlite>>::type_info().eq(_col.type_info()) {
                    let ret = match row.try_get::<$type, _>(_col.name()) {
                        Ok(val) => Some(val),
                        Err(_) => None
                    };

                    result.insert(_col.name().to_string(), $val(ret));
                    continue;
                }
            };
        }

        match_type!(i64, Value::Integer);
        match_type!(String, Value::Text);
        match_type!(NaiveDateTime, Value::DateTime);
    }

    Ok(result)
}