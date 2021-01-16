use std::error::Error;
use std::io;
use rusty_audio::Audio;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};

fn main() -> Result <(), Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode", "wav/explode.wav");
    audio.add("lose", "wav/lose.wav");
    audio.add("move", "wav/move.wav");
    audio.add("pew", "wav/pew.wav");
    audio.add("startup", "wav/startup.wav");
    audio.add("win", "wav/win.wav");
    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
