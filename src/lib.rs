use mpris::PlayerFinder;
use mpris::Player;
use notify_rust::Notification;



// const IMAGE_DOWNLOAD_PATH: &str = "/tmp/image.jpeg";


#[derive(Debug)]
pub struct MusicInfo {
    title: String,
    artist: String,
    icon: String,
   pub playing: String
}

fn get_player() -> Result<Player, Box<dyn std::error::Error>> {
    let player = PlayerFinder::new()?.find_active()?;

  Ok(player)
}

pub fn get_metadata() -> Result<MusicInfo, Box<dyn std::error::Error>> {
    let player = get_player()?;
    let metadata = player.get_metadata()?;
    let title = metadata.title().unwrap_or("Unknown Title").to_string();
    let artists = metadata
    .artists()
    .map(|a|a.join(", "))
    .unwrap_or_else(|| "Unknown Artist".to_string());

    // images aren't currently supported, maybe when the panel gets better support
    // let icon = metadata.art_url().unwrap_or("audio-x-generic").to_string();
    // if icon.contains("https://") {
    //     download_image(&icon,&IMAGE_DOWNLOAD_PATH )?;
    // }

    let song_info = format!("{} by {}", title, artists);
    let music_info = MusicInfo{
        title,
        artist: artists,
        icon: "audio-x-generic".to_string(),
        playing: song_info
    };
    Ok(music_info)
}

// fn download_image(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut resp = reqwest::blocking::get(url)?;
//     let mut out = File::create(path)?;
//     copy(&mut resp, &mut out)?;
//     Ok(())
// }

pub fn notify(info: &MusicInfo) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
    .summary("Now Playing: ")
    .icon(&info.icon)
    // .image_path(&info.icon)
    .body(info.playing.as_str())
    .show()?;
    Ok(())

}

