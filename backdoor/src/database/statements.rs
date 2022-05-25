// start user schema/statements
pub const CREATE_USER_TABLE: &str = "CREATE TABLE IF NOT EXISTS users(
 uuid SERIAL PRIMARY KEY,
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
// end user schema/statements

// start article schema/statements
pub const CREATE_ARTCLE_TABLE: &str = "CREATE TABLE IF NOT EXISTS article(
 uuid SERIAL PRIMARY KEY,
 title VARCHAR(255) NOT NULL,
 content TEXT NOT NULL,
 summary VARCHAR(255),
 slug VARCHAR(255) UNIQUE NOT NULL,
 published BOOL NOT NULL DEFAULT TRUE,
 likes INT DEFAULT 0,
 tags TEXT[],
 updated_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL,
 created_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL
)";

pub const INSERT_ARTICLE: &str =
    "INSERT INTO article (title, content, summary, slug, published, tags) VALUES ($1, $2, $3, $4, $5, $6)";

pub const GET_SINGLE_ARTICLE: &str =
    "SELECT uuid, title, content, summary, slug, created_at, updated_at, likes, published FROM article WHERE slug = $1";

pub const GET_ALL_ARTICLE: &str = "SELECT uuid, title, content, summary, slug, created_at, updated_at, likes, published FROM article";

pub const UPDATE_SINGLE_ARTICLE_TITLE: &str = "UPDATE article SET title = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_CONTENT: &str = "UPDATE article SET content = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_SLUG: &str = "UPDATE article SET slug = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_DESC: &str = "UPDATE article SET summary = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_LIKES: &str = "UPDATE article SET likes = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_PUBLISHED: &str =
    "UPDATE article SET published = $2 WHERE slug = $1";

pub const DELETE_SINGLE_ARTICLE: &str = "DELETE FROM article WHERE slug = $1";
// end article schema/statements
