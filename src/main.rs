use music_info::{get_metadata, notify};
use std::thread::sleep;
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut last_song: Option<String> = None;
    loop {
        let player = get_metadata();
        match player {
            Ok(player) => {
                if Some(&player.playing) != last_song.as_ref() {
                    notify(&player)?;
                    last_song = Some(player.playing);
                }
            }

            Err(_) => {
                sleep(Duration::from_secs(2));
            }
        }
        // println!("{:#?}", player);
        sleep(Duration::from_secs(2));
    }
}
