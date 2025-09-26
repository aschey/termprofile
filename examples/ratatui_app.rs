use std::io::{self, stdout};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::{DefaultTerminal, Frame};
use termprofile::{DetectorSettings, TermProfile};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let profile = TermProfile::detect(&stdout(), DetectorSettings::with_dcs()?);

    loop {
        terminal.draw(|f| draw(&profile, f))?;
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            break Ok(());
        }
    }
}

fn draw(profile: &TermProfile, frame: &mut Frame) {
    let color = Color::Rgb(rand_rgb(), rand_rgb(), rand_rgb());
    let style = profile.adapt_style(Style::new().fg(color));

    frame.render_widget(
        Text::from_iter([
            Line::raw("try using NO_COLOR and FORCE_COLOR to change the output"),
            Line::raw(""),
            Line::styled(
                format!(
                    "random color: {}",
                    style.fg.map(|f| f.to_string()).unwrap_or("N/A".to_string())
                ),
                style,
            ),
        ]),
        frame.area(),
    );
}

fn rand_rgb() -> u8 {
    rand::random_range(0..256) as u8
}
