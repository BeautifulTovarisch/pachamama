use std::env::{ args };

mod watcher;

use watcher::{ parse_args };

fn main() {
    parse_args(args());
}
