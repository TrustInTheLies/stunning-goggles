CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    token TEXT
);
CREATE TABLE IF NOT EXISTS tracks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(50) NOT NULL,
    album VARCHAR(50) NOT NULL,
    artist VARCHAR(50) NOT NULL,
    year SMALLINT NOT NULL,
    genre VARCHAR(30) NOT NULL,
    url VARCHAR(50) NOT NULL,
    image_url VARCHAR(50) NOT NULL
);
CREATE TABLE IF NOT EXISTS users_tracks (
    user_id SMALLINT REFERENCES users(id),
    track_id SMALLINT REFERENCES tracks(id),
    CONSTRAINT users_tracks_id PRIMARY KEY (user_id, track_id)
);