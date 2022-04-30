use anyhow::Result;
use std::env;


#[derive(Debug)]
pub struct Student {
    id: i32,
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    //sqlx postgres
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("111{}", &database_url);

    let mut pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    //sqlx create table
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS student (id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, age INTEGER NOT NULL)").execute(& pool).await;
    match result {
        Ok(res) => println!("ok {:?}", res),
        Err(e) => println!("err{:?}", e),
    }
    //create a new student
    let stu = Student {
        id: 2,
        name: "lisi".to_string(),
        age: 18,
    };
    //sqlx insert
    let result = sqlx::query("INSERT INTO student (name, age) VALUES ($1, $2)")
        .bind(&stu.name)
        .bind(&stu.age)
        .execute(&pool)
        .await;
    match result {
        Ok(res) => println!("ok {:?}", res),
        Err(e) => println!("err{:?}", e),
    }

    //sqlx::query_as!
    let stu = sqlx::query_as!(Student, "SELECT * FROM student")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", stu);

    Ok(())
}
