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
 meta_title VARCHAR(255),
 content TEXT NOT NULL,
 summary VARCHAR(255),
 slug VARCHAR(255) NOT NULL,
 published BOOL NOT NULL DEFAULT TRUE,
 likes INT DEFAULT 0,
 updated_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL,
 created_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL
)";

pub const INSERT_ARTICLE: &str =
    "INSERT INTO article (title, content, summary, slug, published) VALUES ($1, $2, $3, $4, $5)";

pub const GET_SINGLE_ARTICLE: &str =
    "SELECT uuid, title, content, summary, slug, created_at, updated_at, likes, published FROM article WHERE slug = $1";

pub const GET_ALL_ARTICLE: &str = "SELECT uuid, title, content, summary, slug, created_at, updated_at, likes, published FROM article";

pub const UPDATE_SINGLE_ARTICLE_TITLE: &str = "UPDATE article SET title = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_CONTENT: &str = "UPDATE article SET content = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_DESC: &str = "UPDATE article SET summary = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_LIKES: &str = "UPDATE article SET likes = $2 WHERE slug = $1";

pub const UPDATE_SINGLE_ARTICLE_PUBLISHED: &str =
    "UPDATE article SET published = $2 WHERE slug = $1";

pub const DELETE_SINGLE_ARTICLE: &str = "DELETE FROM article WHERE slug = $1";
// end article schema/statements

// start category schema/statments
pub const CREATE_CATEGORY_TABLE: &str = "CREATE TABLE IF NOT EXISTS category(
 uuid SERIAL PRIMARY KEY,
 name VARCHAR(255) UNIQUE NOT NULL,
 created_at timestamp with time zone DEFAULT (now() at time zone 'utc') NOT NULL
)";
// end category schema/statments

// start category schema/statments
pub const CREATE_CATEGORY_ARTICLE_TABLE: &str = "CREATE TABLE IF NOT EXISTS category_article(
 category_id INT REFERENCES category (uuid) ON UPDATE CASCADE ON DELETE CASCADE,
 article_id  INT REFERENCES category (uuid) ON UPDATE CASCADE ON DELETE CASCADE,
 CONSTRAINT category_article_pkey PRIMARY KEY (category_id, article_id)
)";
