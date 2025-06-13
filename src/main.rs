mod modules;

use std::{env, io, process};
use crossterm::event::{KeyCode};
use ratatui::{style::Stylize, text::Line, widgets::{Block, Paragraph}, DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Widget};
use ratatui::symbols::border;
use ratatui::text::Text;
use crate::modules::disassembler::Disassembler::Disassembler;
use crate::modules::rom::Rom;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom = Rom::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problema com o arquivo: {}", err);
        process::exit(1);
    });
    let decoded = Disassembler::disassemble(&*rom.file_name).unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });


    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        instructions: decoded,
        scroll: 0,
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}


pub struct App{
    pub exit: bool,
    pub instructions: Vec<String>,
    pub scroll: u16
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>{
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _=> {}
            }
            terminal.draw( |frame| self.draw(frame));
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()>{
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Down | KeyCode::Char('k') => {
                if self.scroll < (self.instructions.len() as u16).saturating_sub(1) {
                    self.scroll += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('j') => {
                if self.scroll > 0 {
                    self.scroll -= 1;
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" CHIP-8 Disassembler ".bold());
        let instructions = Line::from(vec![
            " Scroll Up ".into(),
            "<J> ".blue().bold(),
            " | ".into(),
            " Scroll Down ".into(),
            "<K> ".blue().bold(),
            " | ".into(),
            " Export as txt ".into(),
            "<T> ".blue().bold(),
            " | ".into(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let s = String::from("Teste de string aqui");
        let decoded = Text::from(
            self.instructions
                .iter()
                .map(|line| Line::from(line.clone()))
                .collect::<Vec<Line>>(),
        );

        Paragraph::new(decoded)
            .block(block)
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}