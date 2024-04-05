use sqlx::postgres::PgPoolOptions;
use std::error::Error;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create_book(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update_book(book: &Book, isbn: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL environment variable error");
    let conn_str = "postgres://superuser:superpassword@0.0.0.0:5432/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book {
        title: "Changed Book".to_string(),
        author: "Changed Author".to_string(),
        isbn: "978-0-385-00751".to_string(),
    };

    update_book(&book, &book.isbn, &pool).await?;

    let res: (i32,) = sqlx::query_as("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    println!("1 + 1 = {}", res.0);
    Ok(())
}
