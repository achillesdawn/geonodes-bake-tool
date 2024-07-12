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
use geonodes_bake_tool::{bake_reader::BakeReader, App};

fn main() {
    let mut reader = BakeReader::new("tests/91383020", &["light", "hit"]);
    let geometry = reader.load_meta().unwrap();

    for (attribute_name, data) in geometry.domain.point.iter() {
        println!("attribute name: {}", attribute_name);
        for frame in data.iter() {
            dbg!(&frame.frame);
            
        }
    }

    let points = geometry.points(1);

    for point in points {
        dbg!(point);
        break;
    }

    
    // let mut config = App::new("/tmp/91383020", vec!["light", "hit"], 88);

    // config.load_meta().unwrap();

    // for frame in config.frames {
    //     print!("{}", frame.buffer);
    // }

    // let mut app = tui::TuiApp::new(config.frames);
    // let mut terminal = tui::init().unwrap();

    // app.run(&mut terminal);
    // tui::restore().unwrap();
}
