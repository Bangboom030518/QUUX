#![feature(is_some_and)]

use std::fs;

fn main() {
    let mut lines = include_str!("input.txt").lines().peekable();
    let mut tags = Vec::new();
    while let Some(line) = lines.next() {
        println!("{line}");
        if let Some(tag_name) = line
            .strip_prefix('<')
            .and_then(|chars| chars.strip_suffix('>'))
        {
            // println!("!! {}", lines.peek().unwrap_or(&"uh oh"));
            if lines.peek().is_some_and(|line: &&str| *line == "Deprecated") {
                lines.next().unwrap();
                continue
            }
            tags.push(tag_name);
            continue
        }
        panic!("Unknown line '{line}'")
    }
    fs::write("output.txt", tags.join(",")).unwrap();
}
