#[macro_export]
macro_rules! update_resource {
    ($resource:ty, $id:expr, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::find_one_resource_where_fields;
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;
        use time::{Duration, OffsetDateTime};

        async {
            let id = $id.to_string();
            let updated_at = OffsetDateTime::now_utc();
            let expires_at = (OffsetDateTime::now_utc() + Duration::days(30));

            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let mut params: Vec<(&str, DatabaseValue)> = Vec::new();

            let input_params: Vec<(&str, DatabaseValue)> = $params;
            if !input_params.is_empty() {
                for (field, value) in input_params {
                    params.push((field, value.clone()));
                }
            }

            if <$resource as DatabaseResource>::is_updatable() {
                if let Some(idx) = params
                    .iter()
                    .position(|(field, _)| field.contains("updated_at"))
                {
                    params[idx] = ("updated_at", updated_at.into());
                } else {
                    params.push(("updated_at", updated_at.into()));
                }
            }

            if <$resource as DatabaseResource>::is_expirable() {
                if let Some(idx) = params
                    .iter()
                    .position(|(field, _)| field.contains("expires_at"))
                {
                    params[idx] = ("expires_at", expires_at.into());
                } else {
                    params.push(("expires_at", expires_at.into()));
                }
            }

            let fields = params
                .iter()
                .map(|(field, _)| field.to_string())
                .collect::<Vec<String>>();
            let values: Vec<&DatabaseValue> = params.iter().map(|(_, value)| value).collect();

            let mut query = format!("UPDATE {} SET ", resource_name);

            for (i, field) in fields.iter().enumerate() {
                let value = values[i];
                match value {
                    DatabaseValue::None => {
                        query.push_str(&format!("{} = NULL", field));
                    }
                    DatabaseValue::Str(_) | DatabaseValue::String(_) | DatabaseValue::Text(_) => {
                        query.push_str(&format!("{} = ${}", field, i + 1));
                    }
                    DatabaseValue::DateTime(_) => {
                        query.push_str(&format!(
                            "{} = CAST(${} AS TIMESTAMP WITH TIME ZONE)",
                            field,
                            i + 1
                        ));
                    }
                    DatabaseValue::Int(_) => {
                        query.push_str(&format!("{} = CAST(${} AS INTEGER)", field, i + 1));
                    }
                    DatabaseValue::Int32(_) => {
                        query.push_str(&format!("{} = CAST(${} AS INTEGER)", field, i + 1));
                    }
                    DatabaseValue::Int64(_) => {
                        query.push_str(&format!("{} = CAST(${} AS BIGINT)", field, i + 1));
                    }
                    DatabaseValue::Float(_) => {
                        query.push_str(&format!("{} = CAST(${} AS FLOAT)", field, i + 1));
                    }
                    DatabaseValue::Boolean(_) => {
                        query.push_str(&format!("{} = CAST(${} AS BOOLEAN)", field, i + 1));
                    }
                }
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }

            query.push_str(&format!(" WHERE id = ${}", fields.len() + 1));
            query.push_str(&format!(" RETURNING *"));

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                match value {
                    DatabaseValue::None => query = query.bind(Option::<String>::None),
                    _ => query = query.bind(value),
                }
            }
            query = query.bind(&id);

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            let params: Vec<(&str, DatabaseValue)> = vec![("id", $id.into())];
            match find_one_resource_where_fields!($resource, params).await {
                Ok(resource) => Ok(resource),
                Err(e) => Err(e),
            }
        }
    }};
}
