#[macro_export]
macro_rules! insert_resource {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;
        use time::{Duration, OffsetDateTime};
        use uuid::Uuid;

        let input_params: Vec<(&str, DatabaseValue)> = $params;
        async {
            let id = Uuid::new_v4().to_string();
            let created_at = OffsetDateTime::now_utc();
            let updated_at = created_at.clone();
            let expires_at = (OffsetDateTime::now_utc() + Duration::days(30));

            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let mut params: Vec<(String, DatabaseValue)> = Vec::new();
            for (field, value) in input_params.into_iter() {
                params.push((field.to_string(), value.clone()))
            }

            if <$resource as DatabaseResource>::has_id() {
                params.push(("id".to_string(), id.clone().into()));
            }

            if <$resource as DatabaseResource>::is_creatable() {
                if let Some(idx) = params
                    .iter()
                    .position(|(field, _)| field.contains("created_at"))
                {
                    params[idx] = (
                        "created_at".to_string(),
                        DatabaseValue::DateTime(created_at.clone().to_string()),
                    );
                } else {
                    params.push((
                        "created_at".to_string(),
                        DatabaseValue::DateTime(created_at.clone().to_string()),
                    ));
                }
            }

            if <$resource as DatabaseResource>::is_updatable() {
                if let Some(idx) = params
                    .iter()
                    .position(|(field, _)| field.contains("updated_at"))
                {
                    params[idx] = (
                        "updated_at".to_string(),
                        DatabaseValue::DateTime(updated_at.clone().to_string()),
                    );
                } else {
                    params.push((
                        "updated_at".to_string(),
                        DatabaseValue::DateTime(updated_at.clone().to_string()),
                    ));
                }
            }

            if <$resource as DatabaseResource>::is_expirable() {
                if let Some(idx) = params
                    .iter()
                    .position(|(field, _)| field.contains("expires_at"))
                {
                    params[idx] = (
                        "expires_at".to_string(),
                        DatabaseValue::DateTime(expires_at.clone().to_string()),
                    );
                } else {
                    params.push((
                        "expires_at".to_string(),
                        DatabaseValue::DateTime(expires_at.clone().to_string()),
                    ));
                }
            }

            let fields: Vec<String> = params.iter().map(|(field, _)| field.clone()).collect();
            let values: Vec<DatabaseValue> =
                params.iter().map(|(_, value)| (*value).clone()).collect();

            let mut query = format!("INSERT INTO {} (", resource_name);

            for (i, field) in fields.iter().enumerate() {
                query.push_str(field);
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }

            query.push_str(") VALUES (");
            for (i, value) in values.iter().enumerate() {
                match value {
                    DatabaseValue::None => {
                        query.push_str("NULL");
                    }
                    DatabaseValue::Str(_) | DatabaseValue::String(_) => {
                        query.push_str(&format!("Cast(${} AS VARCHAR)", i + 1));
                    }
                    DatabaseValue::Text(_) => {
                        query.push_str(&format!("Cast(${} AS TEXT)", i + 1));
                    }
                    DatabaseValue::DateTime(_) => {
                        query.push_str(&format!("CAST(${} AS TIMESTAMP WITHOUT TIME ZONE)", i + 1));
                    }
                    DatabaseValue::Int(_) => {
                        query.push_str(&format!("CAST(${} AS INTEGER)", i + 1));
                    }
                    DatabaseValue::Int32(_) => {
                        query.push_str(&format!("CAST(${} AS INTEGER)", i + 1));
                    }
                    DatabaseValue::Int64(_) => {
                        query.push_str(&format!("CAST(${} AS BIGINT)", i + 1));
                    }
                    DatabaseValue::Float(_) => {
                        query.push_str(&format!("CAST(${} AS FLOAT)", i + 1));
                    }
                    DatabaseValue::Boolean(_) => {
                        query.push_str(&format!("CAST(${} AS BOOLEAN)", i + 1));
                    }
                }
                if i < values.len() - 1 {
                    query.push_str(", ");
                }
            }
            query.push_str(") RETURNING *");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.fetch_one(&pool).await {
                Ok(row) => Ok(<$resource as DatabaseResource>::from_row(&row)?),
                Err(e) => {
                    println!("Error fetching row: {:?}", e);
                    Err(e)
                }
            }
        }
    }};
}
