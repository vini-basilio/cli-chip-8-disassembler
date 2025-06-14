mod modules;

use std::{env, io, process};
use std::io::prelude::*;
use ratatui::{style::Stylize};
use ratatui::prelude::{Widget};
use crate::modules::disassembler::lib::disassembler;
use crate::modules::rom::Rom;
use crate::modules::ui::App;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom = Rom::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("File problem: {}", err);
        process::exit(1);
    });


    let decoded = disassembler::run(&*rom.file_name).unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });


    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        instructions: decoded,
        scroll: 0,
        rom_path: (&*rom.file_name).parse().unwrap(),
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
