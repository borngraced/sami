pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS  users(
 uuid SERIAL PRIMARY KEY NOT NULL,
 email VARCHAR(255) UNIQUE NOT NULL,
 username VARCHAR(255) UNIQUE NOT NULL,
 password VARCHAR(255) NOT NULL,
 role VARCHAR(255) NOT NULL,
 created_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL
)";

pub const INSERT_USER: &str =
    "INSERT INTO users (username, email, password, role) VALUES ($1, $2, $3, $4)";

pub const GET_SINGLE_USER: &str =
    "SELECT uuid, email, username, created_at, role FROM users WHERE uuid = $1";
