use std::io::Write;
use std::fs::File;
use std::io;
use std::path::Path;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize, Text, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

pub struct App{
    pub exit: bool,
    pub instructions: Vec<String>,
    pub scroll: u16,
    pub rom_path: String,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>{
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
            KeyCode::Char('s') => {
                if let Err(err) = self.export_as_txt() {
                    eprintln!("Erro to export: {}", err);
                }
                self.exit = true;
            }
            _ => {}
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }
    fn export_as_txt(&mut self) -> std::io::Result<()> {

        let rom_path = Path::new(&self.rom_path);
        let output_path = rom_path.with_extension("txt");
        
        let mut file = File::create(&output_path)?;
        
        for line in &self.instructions {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" CHIP-8 Disassembler ".bold());
        let instructions = Line::from(vec![
            " Scroll Up ".into(),
            "<j> ".blue().bold(),
            " | ".into(),
            " Scroll Down ".into(),
            "<k> ".blue().bold(),
            " | ".into(),
            " Export as txt and Exit".into(),
            "<s> ".blue().bold(),
            " | ".into(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let mut lines = vec![];

        let header = "Hello, this is a CHIP-8 disassembler!\n\
It is based on Tania's idea, so check her blog, link on meu github.\n\
\n\
Let's set your session. CHIP-8 has a lot of versions, so we\n\
need to choose what to support. For all opcodes not supported, disassembler\n\
will return '0xNNNN not founded'.\n\
\n\
Currently, only 'Cowgod's Chip-8 Technical Reference v1.0' assembly\n\
is supported, covering standard and Super Chip-48. All instructions\n\
of the Super version will be followed by [super set].\n";

        lines.extend(header.lines().map(|l| Line::from(l.to_string())));
        lines.push(Line::from("")); // linha vazia entre header e instruções
        lines.extend(self.instructions.iter().map(|line| Line::from(line.clone())));

        let decoded = Text::from(lines);

        Paragraph::new(decoded)
            .block(block)
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}