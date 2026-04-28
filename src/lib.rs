use mpris::Player;
use mpris::PlayerFinder;
use notify_rust::Notification;
use std::path::Path;
pub mod utils;

const IMAGE_PATH: &str = "/tmp/";

#[derive(Debug)]
pub struct MusicInfo {
    title: String,
    artist: String,
    icon: String,
    image_path: Option<String>,
    pub playing: String,
}

fn get_player() -> Result<Player, Box<dyn std::error::Error>> {
    let player = PlayerFinder::new()?.find_active()?;

    Ok(player)
}

pub fn get_metadata(
    config: &utils::Config,
) -> Result<Option<MusicInfo>, Box<dyn std::error::Error>> {
    let player = match get_player() {
        Ok(p) => p,
        Err(_) => return Ok(None),
    };
    let metadata = player.get_metadata()?;

    let title = match metadata.title() {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return Ok(None),
    };
    let artists = match metadata.artists() {
        Some(a) if !a.is_empty() => a.join(", "),
        _ => return Ok(None),
    };

    let icon = metadata.art_url().unwrap_or("audio-x-generic").to_string();

    let image_path = if config.album_art && icon.contains("https://") {
        Some(download_image(
            &icon,
            &IMAGE_PATH,
            &title,
            artists.as_str(),
        )?)
    } else {
        None
    };

    let song_info = format!("{} by {}", title, artists);
    let music_info = MusicInfo {
        title,
        artist: artists,
        icon: icon,
        playing: song_info,
        image_path,
    };
    Ok(Some(music_info))
}

// TODO: maybe hash the images instead
fn download_image(
    url: &str,
    path: &str,
    title: &str,
    artist: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = format!("{}{}-{}.jpeg", path, sanitize(title), sanitize(artist));

    if Path::new(&file_name).exists() {
        return Ok(file_name);
    }
    let mut response = ureq::get(url).call()?;
    let mut reader = response.body_mut().as_reader();
    let mut file = std::fs::File::create(&file_name)?;
    std::io::copy(&mut reader, &mut file)?;

    Ok(file_name)
}

pub fn notify(info: &MusicInfo) -> Result<(), Box<dyn std::error::Error>> {
    let image_uri = info
        .image_path
        .as_ref()
        .map(|p| format!("file://{}", p))
        .unwrap_or_else(|| "audio-x-generic".to_string());

    Notification::new()
        .summary("Now Playing:")
        .icon("audio-x-generic")
        .image_path(&image_uri)
        .body(&info.playing)
        .show()?;

    Ok(())
}

fn sanitize(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace('/', "_")
        .replace('\\', "_")
        .replace(':', "_")
        .replace(' ', "_")
}
