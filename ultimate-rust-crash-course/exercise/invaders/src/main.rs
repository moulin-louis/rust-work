use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

fn setup_audio(audio:&mut Audio) {
	audio.add("explode", "./sound/explode.wav");
	audio.add("lose", "./sound/lose.wav");
	audio.add("move", "./sound/move.wav");
	audio.add("pew", "./sound/pew.wav");
	audio.add("startup", "./sound/startup.wav");
	audio.add("win", "./sound/win.wav");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
	setup_audio(&mut audio);
	audio.play("startup");

	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(EnterAlternateScreen)?;
	stdout.execute(Hide)?;

	'gameloop: loop {
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
	}

	audio.wait();
	stdout.execute(Show)?;
	stdout.execute(LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;
	Ok(())
}
