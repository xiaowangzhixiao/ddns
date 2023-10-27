
use rusqlite::{Connection, Result};

const CREATE_TABLE: &str = "CREATE TABLE ipv6_info
                             (id INTEGER PRIMARY KEY,
                             name TEXT NOT NULL,
                             ipv6 TEXT NOT NULL)";

fn main() -> Result<()> {
    let conn = Connection::open("./ddns.bin")?;
    conn.execute(CREATE_TABLE,
        (), // empty list of parameters.
    )?;
    Ok(())
}
