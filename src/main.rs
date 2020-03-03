use std::env::{ args };

mod parser;
mod watcher;

use watcher::{ watch };

fn main() {
    watch(args());
}
