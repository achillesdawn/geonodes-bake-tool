use std::io::{self, stdout, Stdout};

use crossterm::event::Event;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, ToLine},
    widgets::{
        block::{Position, Title},
        Block, List, Paragraph, Widget,
    },
    Terminal,
};

use crate::api::{self};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub struct TuiApp {
    done: bool,
    is_playing: bool,
    current_frame: usize,
    total_frames: usize,
}

impl TuiApp {
    pub fn new() -> Self {
        TuiApp {
            done: false,
            is_playing: true,
            current_frame: 0,
            total_frames: frames.len(),
            frames,
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.done {
            terminal.draw(|frame| {
                self.render_frame(frame);
            })?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {}
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Left => self.set_current_frame(false),
                    KeyCode::Right => self.set_current_frame(true),
                    KeyCode::Char(' ') => self.set_is_playing(),
                    KeyCode::Char('q') => {
                        self.done = true;
                    }
                    _ => {}
                };
            }
            _ => {}
        }

        Ok(())
    }

    fn set_current_frame(&mut self, increment: bool) {
        if increment {
            self.current_frame += 1;
            self.current_frame = self.current_frame.rem_euclid(self.total_frames);
        } else {
            if self.current_frame == 0 {
                self.current_frame = self.total_frames - 1;
            } else {
                self.current_frame -= 1;
            }
        }
    }

    fn set_is_playing(&mut self) {
        self.is_playing = !self.is_playing;
    }
}

impl Widget for &TuiApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(91)])
            .split(area);

        let title = Title::from("TUI").alignment(Alignment::Center);
        let commands = Title::from(Line::from(vec![format!(
            ".:: Frame {} ::.",
            self.current_frame
        )
        .bold()]))
        .alignment(Alignment::Center)
        .position(Position::Bottom);

        let block = Block::bordered()
            .title(title)
            .title(commands)
            .border_set(border::ROUNDED)
            .cyan();

        // let frame = self.frames.get(self.current_frame).unwrap();
        // let p = Paragraph::new(frame.buffer.clone())
        //     .wrap(ratatui::widgets::Wrap { trim: false })
        //     .centered()
        //     .block(block);

        // p.render(layout[1], buf);

        // let list_items: Vec<_> = self
        //     .frames
        //     .iter()
        //     .map(|frame| frame.number.to_line())
        //     .collect();

        // List::new(list_items)
        //     .highlight_style(Style::default().fg(Color::White))
        //     .render(layout[0], buf);
    }
}
