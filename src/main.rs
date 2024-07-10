#![allow(unused)]

use std::{
    fs,
    io::{self, stdout, Read, Seek, Write},
    path::PathBuf,
    str::FromStr,
    thread::sleep,
    time::Duration,
};

use crossterm::QueueableCommand;
use geonodes_bake_tool::{tui, App};

fn main() {
    let mut config = App::new("/tmp/91383020", vec!["light", "hit"], 88);

    config.load_meta().unwrap();

    // for frame in config.frames {
    //     print!("{}", frame.buffer);
    // }

    // let mut app = tui::TuiApp::new(config.frames);
    // let mut terminal = tui::init().unwrap();

    // app.run(&mut terminal);
    // tui::restore().unwrap();
}
