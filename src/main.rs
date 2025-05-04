use ratatui::crossterm::event::{self, Event};
use std::io;
use tachyonfx::EffectRenderer;
use tachyonfx::Shader;

mod app;
mod game;
mod ui;

use app::App;
use ui::ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(); // Using new() with explicit initialization

    // Main loop moved to main.rs
    while !app.should_quit() {
        terminal.draw(|frame| {
            ui(frame, &mut app);
            if let Some(effect) = &mut app.effect {
                let elapsed = app.last_frame.elapsed();
                let area = frame.area();
                frame.render_effect(effect, area, elapsed.into());
                if !effect.running() {
                    app.effect = None;
                }
            }
        })?;
        app.last_frame = std::time::Instant::now();
        if let Event::Key(key) = event::read()? {
            app.handle_event(key);
        }
    }

    ratatui::restore();
    Ok(())
}
