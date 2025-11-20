#[macro_export]
macro_rules! find_all_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params
                .iter()
                .map(|field| field.1.clone())
                .collect::<Vec<DatabaseValue>>();

            let mut query = format!("SELECT * FROM {}", resource_name);
            if fields.len() > 0 {
                query.push_str(" WHERE ");
            }
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(" ORDER BY updated_at DESC");

            let mut query = sqlx::query(&query);
            for value in values.iter() {
                query = query.bind(value);
            }

            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_all_unarchived_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params.iter().map(|field| &field.1).collect::<Vec<_>>();

            let mut query = format!("SELECT * FROM {} WHERE archived_at IS NULL", resource_name);
            if fields.len() > 0 {
                query.push_str(" AND ");
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

            query.push_str(" ORDER BY created_at DESC");

            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_all_archived_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL",
                resource_name
            );
            if fields.len() > 0 {
                query.push_str(" AND ");
            }
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }

            query.push_str(" ORDER BY created_at DESC");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {}", resource_name);
            if fields.len() > 0 {
                query.push_str(" WHERE ");
            }
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(" LIMIT 1");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.fetch_one(&pool).await {
                Ok(row) => <$resource as DatabaseResource>::from_row(&row),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_unarchived_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {} WHERE archived_at IS NULL", resource_name);
            if fields.len() > 0 {
                query.push_str(" AND ");
            }
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(" LIMIT 1");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.fetch_one(&pool).await {
                Ok(row) => <$resource as DatabaseResource>::from_row(&row),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_archived_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL",
                resource_name
            );
            if fields.len() > 0 {
                query.push_str(" AND ");
            }

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();

            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(" LIMIT 1");

            let mut query = sqlx::query(&query);
            for (_, value) in params.iter().enumerate() {
                query = query.bind(value.1.clone());
            }

            match query.fetch_one(&pool).await {
                Ok(row) => Ok(<$resource as DatabaseResource>::from_row(&row)?),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_all_resources_where_fields_like {
    ($resource:ty, $params:expr, $search_term:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let params: Vec<&str> = $params.clone();

            let mut query = format!("SELECT * FROM {}", resource_name);
            if params.len() > 0 {
                query.push_str(" WHERE ");
            }
            for (i, field) in params.iter().enumerate() {
                query.push_str(&format!("{} ILIKE ${}", field, i + 1));
                if i < params.len() - 1 {
                    query.push_str(" OR ");
                }
            }

            query.push_str(" ORDER BY created_at DESC");

            let mut query = sqlx::query(&query);
            for _ in params.iter() {
                query = query.bind(format!("%{}%", $search_term));
            }

            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}
