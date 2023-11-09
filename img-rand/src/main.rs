use rand::seq::IteratorRandom;
use std::fs;
use std::process::Command;

fn main() {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir("/directory/to/use").unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let path = file.path().display().to_string();
    Command::new("ristretto").arg(path).spawn().expect("Image viewer faild to start");
}
