use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode", "wav/explode.wav");
    audio.add("lose", "wav/lose.wav");
    audio.add("move", "wav/move.wav");
    audio.add("pew", "wav/pew.wav");
    audio.add("startup", "wav/startup.wav");
    audio.add("win", "wav/win.wav");
    audio.play("startup");

    //Cleanup
    audio.wait();
    Ok(())
}
