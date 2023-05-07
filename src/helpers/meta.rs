use id3::{Tag, TagLike};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    title: String,
    album: String,
    artist: String,
    genre: String,
    year: u16,
    url: String,
    image_url: String,
}

pub fn write_file(data: Vec<Track>) {
    let file = std::fs::File::create("data.json").unwrap();
    serde_json::to_writer_pretty(file, &data).unwrap();
}

pub fn get_data(music: Vec<PathBuf>, covers: Vec<PathBuf>) -> Vec<Track> {
    let mut data = vec![];
    music.iter().for_each(|f| {
        let tags = Tag::read_from_path(f).unwrap();
        let track = Track {
            title: tags
                .title()
                .map_or("Unknown".to_owned(), |value| value.to_owned()),
            album: tags
                .album()
                .map_or("Unknown".to_owned(), |value| value.to_owned()),
            artist: tags
                .artist()
                .map_or("Unknown".to_owned(), |value| value.to_owned()),
            genre: tags
                .genre()
                .map_or("Unknown".to_owned(), |value| value.to_owned()),
            year: tags.year().map_or(0, |value| value as u16),
            url: format!(
                "http://localhost:3000/static/music/{}",
                f.file_name().unwrap().to_str().unwrap()
            ),
            image_url: format!(
                "http://localhost:3000/static/covers/{}",
                covers
                    .iter()
                    .find(|file| file.file_stem() == f.file_stem())
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
        };
        data.push(track);
    });
    data
}

pub fn get_paths(path: &str) -> Vec<PathBuf> {
    let files = std::fs::read_dir(path).unwrap();
    let mut paths = vec![];
    files.for_each(|f| {
        paths.push(f.unwrap().path());
    });
    paths
}
