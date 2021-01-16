use std::io;
use std::error::Error;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::thread;

use frame::new_frame;
use render::render;
use rusty_audio::Audio;

use crossterm::{event, terminal, ExecutableCommand};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{Event, KeyCode};

use invaders::frame;
use invaders::render;

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

    //Render Loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game Loop
    'gameloop: loop {
        //Per Frame Init
        let curr_frame = new_frame();


        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}
                }
            }
        }
        //Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
