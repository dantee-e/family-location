use crate::error::ServerError;
use crate::structs::Location;

use super::structs::RegisterUserInfo;
use ed25519_dalek::VerifyingKey;
use rusqlite::{Connection, Error, Result};

static DB_FILE_PATH: &str = "src/sql/data.db";

pub fn check_db_exists() -> bool {
    std::fs::metadata(DB_FILE_PATH).is_ok()
}

fn check_table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_schema WHERE type='table' AND name=?",
    )?;
    if let Err(Error::QueryReturnedNoRows) =
        stmt.query_row([table_name], |row| row.get::<_, i64>(0))
    {
        return Ok(false);
    }
    Ok(true)
}

pub fn get_all_users() -> Result<Vec<RegisterUserInfo>> {
    let conn = Connection::open(DB_FILE_PATH).unwrap();
    let comando = "SELECT verifying_key, public_key, username FROM users";
    let mut stmt = conn.prepare(comando)?;
    let mut rows = stmt.query(())?;

    let mut users: Vec<RegisterUserInfo> = Vec::new();
    while let Some(row) = rows.next()? {
        let user = RegisterUserInfo {
            verifying_key: row.get(0).unwrap(),
            public_key: row.get(1).unwrap(),
            username: row.get(2).unwrap(),
        };
        users.push(user);
    }

    Ok(users)
}

pub fn check_user_exists_from_verifying_key(
    verifying_key: VerifyingKey,
) -> Result<Option<String>> {
    let verifying_key_bytes = verifying_key.to_bytes();
    let conn = Connection::open(DB_FILE_PATH)?;

    let comando =
        "SELECT verifying_key, username FROM users WHERE (verifying_key = ?)";

    match conn.query_row(comando, [verifying_key_bytes], |row| row.get(1)) {
        Ok(username) => Ok(Some(username)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => {
            println!("error: {e}");
            Err(e)
        }
    }
}

pub fn get_locations_from_username(
    username: String,
) -> Result<Vec<Location>, ServerError> {
    let ver_key_bytes = verifying_key_from_username(username)?;
    get_locations_from_verifying_key(&ver_key_bytes)
}

pub fn get_locations_from_verifying_key(
    verifying_key: &[u8; 32],
) -> Result<Vec<Location>, ServerError> {
    let conn = Connection::open(DB_FILE_PATH).unwrap();
    let comando = "SELECT verifying_key, payload, signature, nonce, received_at FROM locations WHERE (verifying_key = ?)";
    let mut stmt = conn.prepare(comando)?;
    let mut rows = stmt.query([verifying_key])?;

    let mut locations: Vec<Location> = Vec::new();
    while let Some(row) = rows.next()? {
        let location = Location {
            verifying_key: row.get(0).unwrap(),
            payload: row.get(1).unwrap(),
            signature: row.get(2).unwrap(),
            nonce: row.get(3).unwrap(),
        };
        locations.push(location);
    }

    Ok(locations)
}

pub fn verifying_key_from_username(
    username: String,
) -> std::result::Result<[u8; 32], ServerError> {
    let conn = Connection::open(DB_FILE_PATH)?;
    let comando =
        "SELECT verifying_key, username FROM users WHERE (username = ?)";

    match conn.query_row(comando, [username], |row| row.get(0)) {
        Ok(v_key_vec) => Ok(v_key_vec),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err(ServerError::UserNotFound)
        }
        Err(e) => {
            println!("error: {e}");
            Err(ServerError::SqliteError(e))
        }
    }
}

pub fn create_db() {
    let comando = std::fs::read_to_string("src/sql/comando_criacao_dbs.sql")
        .expect("Arquivo nao encontrado");
    let conn = Connection::open(DB_FILE_PATH)
        .expect("Failed to create connection to db");

    let (locations, users, access_grants) = (
        check_table_exists(&conn, "locations").unwrap(),
        check_table_exists(&conn, "users").unwrap(),
        check_table_exists(&conn, "access_grants").unwrap(),
    );

    if locations && users && access_grants {
        return;
    }

    println!("Criando database");

    conn.execute_batch(&comando)
        .expect("Failed the create database command");
}

pub fn submit_new_location(
    data: &Location,
) -> std::result::Result<(), ServerError> {
    let conn = Connection::open(DB_FILE_PATH)?;

    conn.execute(
        "INSERT INTO locations (verifying_key, payload, signature, nonce) VALUES (?1, ?2, ?3, ?4)",
        (&data.verifying_key, &data.payload, &data.signature, &data.nonce),
    )?;

    Ok(())
}

pub fn register_new_user(user: RegisterUserInfo) -> Result<()> {
    let conn = Connection::open(DB_FILE_PATH)?;

    conn.execute(
        "INSERT INTO users (username, verifying_key, public_key) VALUES (?1, ?2, ?3)",
        (user.username, user.verifying_key, user.public_key),
    )?;

    Ok(())
}
