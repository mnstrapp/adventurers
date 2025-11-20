#[macro_export]
macro_rules! delete_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::connection::get_connection;
        use crate::database::traits::DatabaseResource;
        use crate::database::values::DatabaseValue;
        use crate::utils::strings::camel_to_snake_case;
        use anyhow::anyhow;
        use pluralizer::pluralize;
        use time::OffsetDateTime;

        async {
            let archived_at = OffsetDateTime::now_utc();

            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();

            let fields: Vec<String> = params.iter().map(|field| field.0.to_string()).collect();
            let values: Vec<DatabaseValue> = params.iter().map(|field| field.1.clone()).collect();

            let mut query: String;
            if <$resource as DatabaseResource>::is_archivable() {
                query = format!(
                    "UPDATE {} SET archived_at = CAST(${} AS TIMESTAMP WITH TIME ZONE) WHERE ",
                    resource_name,
                    fields.len() + 1
                );
            } else {
                query = format!("DELETE FROM {} WHERE ", resource_name);
            }

            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            if <$resource as DatabaseResource>::is_archivable() {
                query = query.bind(archived_at);
            }

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(anyhow!(e)),
            };

            Ok(())
        }
    }};
    ($resource:ty, $params:expr, $permanent:expr) => {{
        use crate::database::connection::get_connection;
        use crate::database::traits::DatabaseResource;
        use crate::database::values::DatabaseValue;
        use crate::utils::strings::camel_to_snake_case;
        use anyhow::anyhow;
        use pluralizer::pluralize;
        use time::OffsetDateTime;

        async {
            let archived_at = OffsetDateTime::now_utc();

            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let permanent: bool = $permanent;
            let params: Vec<(&str, DatabaseValue)> = $params.clone();

            let fields: Vec<String> = params.iter().map(|field| field.0.to_string()).collect();
            let values: Vec<DatabaseValue> = params.iter().map(|field| field.1.clone()).collect();

            let mut query: String;
            if permanent {
                query = format!("DELETE FROM {} WHERE ", resource_name);
            } else {
                if <$resource as DatabaseResource>::is_archivable() {
                    query = format!(
                        "UPDATE {} SET archived_at = CAST(${} AS TIMESTAMP WITH TIME ZONE) WHERE ",
                        resource_name,
                        fields.len() + 1
                    );
                } else {
                    query = format!("DELETE FROM {} WHERE ", resource_name);
                }
            }

            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            if !permanent && <$resource as DatabaseResource>::is_archivable() {
                query = query.bind(archived_at);
            }

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(anyhow!(e)),
            };

            Ok(())
        }
    }};
}
