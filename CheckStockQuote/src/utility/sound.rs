use std::io::BufReader;
use std::thread;
use std::time::Duration;
use rodio;

pub fn play_beep(){
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    for _ in 1..=10{
        let file = std::fs::File::open("beep-sound.wav").unwrap();
        let beep1 = stream_handle.play_once(BufReader::new(file)).unwrap();
        beep1.set_volume(0.2);
        thread::sleep(Duration::from_millis(500));
    }
}