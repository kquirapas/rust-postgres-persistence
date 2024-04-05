use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::error::Error;

#[derive(Debug)]
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

async fn read(pool: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);

    let row = query.fetch_one(pool).await?;

    Ok(Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    })
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

    let one_book = read(&pool).await?;

    println!("{:?}", one_book);

    Ok(())
}
