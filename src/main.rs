use music_info::{get_metadata, notify};
use std::thread::sleep;
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut last_song: Option<String> = None;
    loop {
        let info = get_metadata();
        match info {
            Ok(info) => {
                if last_song.as_deref() != Some(&info.playing) {
                    notify(&info)?;
                    last_song = Some(info.playing);
                }
            }

            Err(_) => {
                last_song = None;
            }
        }
        sleep(Duration::from_secs(2));
    }
}
