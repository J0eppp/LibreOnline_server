use r2d2_sqlite::SqliteConnectionManager;
use r2d2;

pub fn open() -> Result<r2d2::Pool<SqliteConnectionManager>, r2d2::Error> {
    r2d2::Pool::new(SqliteConnectionManager::file("./libreonline.db"))
}

pub fn setup(conn: r2d2::PooledConnection<SqliteConnectionManager>) -> bool {
    // Setup DB
    match conn.execute(
        "DROP TABLE IF EXISTS clients;
            DROP TABLE IF EXISTS files;
            CREATE TABLE clients (
                id INTEGER PRIMARY KEY,
                token TEXT NOT NULL
            );
            CREATE TABLE files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL  
            );
            ",
            [],
    ) {
        Ok(_) => { }
        Err(err) => {
            println!("Error: {}", err);
            return false
        }
    }

    true
}