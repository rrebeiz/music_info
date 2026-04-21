fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut last_song: Option<String> = None;
    loop {

        let player = music_info::get_metadata()?;
        // println!("{:#?}", player);
        if Some(&player.playing) != last_song.as_ref() {
        music_info::notify(&player)?;
        last_song = Some(player.playing);
        }
    std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
