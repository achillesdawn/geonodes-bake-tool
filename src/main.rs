use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
struct Config {
    base_path: PathBuf,
}

fn read_meta(path: PathBuf) {

}

impl Config {
    
    fn load_meta(&self) {
        let mut meta_path = self.base_path.clone();
        meta_path.push("meta");

        for entry in std::fs::read_dir(meta_path).unwrap() {
            let entry = entry.unwrap();

            let metadata = entry.metadata().unwrap();

            if metadata.is_file() {
                dbg!(entry);
            }
        }
    }
}
fn main() {
    let config = Config {
        base_path: PathBuf::from_str("/tmp/91383020").unwrap(),
    };

    config.load_meta();

}
