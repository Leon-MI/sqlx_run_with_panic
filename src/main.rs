use sqlx::SqlitePool;

pub async fn get_all_parent_macro(pool: &SqlitePool, node_id: i64) -> Result<String, sqlx::Error> {
    let path: String = sqlx::query_scalar!(
        r#"
    WITH RECURSIVE path(level, name, parent_id) AS (
        SELECT 0, name, parent_id FROM node WHERE id = ?
        UNION ALL
        SELECT path.level + 1, node.name, node.parent_id FROM node
        JOIN path ON node.id = path.parent_id
    ), ordered_path AS (
        SELECT name FROM path
        order by level DESC
    )
    SELECT group_concat(name, '/') FROM ordered_path
    "#,
        node_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(path)
}

pub async fn get_path_no_macro(pool: &SqlitePool, node_id: i64) -> Result<String, sqlx::Error> {
    let path = sqlx::query_scalar(
        r#"
    WITH RECURSIVE path(level, name, parent_id) AS (
        SELECT 0, name, parent_id FROM node WHERE id = ?
        UNION ALL
        SELECT path.level + 1, node.name, node.parent_id FROM node
        JOIN path ON node.id = path.parent_id
    ), ordered_path AS (
        SELECT name FROM path
        order by level DESC
    )
    SELECT group_concat(name, '/') FROM ordered_path
    "#,
    )
    .bind(node_id)
    .fetch_one(pool)
    .await?;

    Ok(path)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://test.db")
        .await
        .expect("No db connection");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let result = get_path_no_macro(&pool, 2).await.unwrap();
    assert_eq!("root_node/child_node", result);
}
