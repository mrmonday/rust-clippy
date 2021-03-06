#![feature(plugin)]
#![plugin(clippy)]
use std::fs::OpenOptions;

#[allow(unused_must_use)]
#[deny(nonsensical_open_options)]
fn main() {
	OpenOptions::new().read(true).truncate(true).open("foo.txt"); //~ERROR File opened with "truncate" and "read"
	OpenOptions::new().append(true).truncate(true).open("foo.txt"); //~ERROR File opened with "append" and "truncate"
    
	OpenOptions::new().read(true).read(false).open("foo.txt"); //~ERROR The method "read" is called more than once
	OpenOptions::new().create(true).create(false).open("foo.txt"); //~ERROR The method "create" is called more than once
	OpenOptions::new().write(true).write(false).open("foo.txt"); //~ERROR The method "write" is called more than once
	OpenOptions::new().append(true).append(false).open("foo.txt"); //~ERROR The method "append" is called more than once
	OpenOptions::new().truncate(true).truncate(false).open("foo.txt"); //~ERROR The method "truncate" is called more than once
}
