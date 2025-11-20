#[macro_export]
macro_rules! join_all_resources_where_fields_on {
    ($resource:ty, $join_resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = camel_to_snake_case(stringify!($resource).to_string());
            let resource_table_name = pluralize(&resource_name, 2, false);
            let resource_join_name = format!("{}_id", resource_name);

            let join_resource_name = camel_to_snake_case(stringify!($join_resource).to_string());
            let join_resource_table_name = pluralize(&join_resource_name, 2, false);
            let join_resource_join_name = format!("{}_id", join_resource_name);

            let pool = get_connection().await;

            let params: Vec<(&str, DatabaseValue)> = $params.clone();
            let fields = params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = params
                .iter()
                .map(|field| field.1.to_string())
                .collect::<Vec<String>>();

            let mut query = format!(
                "SELECT * FROM {} JOIN {} ON {} = {}",
                resource_table_name,
                join_resource_table_name,
                join_resource_join_name,
                resource_join_name
            );

            query.push_str(" WHERE ");
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

            match query.fetch_all(&pool).await {
                Ok(rows) => Ok(rows
                    .iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(row).unwrap())
                    .collect::<Vec<$resource>>()),
                Err(e) => Err(e),
            }
        }
    }};
}
