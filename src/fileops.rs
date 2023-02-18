use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;

pub fn readfile(file_name: String) {

	let mut file = File::open(file_name)
	.expect("can't open file");

	let mut contents = String::new();

	file.read_to_string(&mut contents)
	.expect("can't read file");

	println!("{}", contents);
}

pub fn writefile(file_name: String) {

	println!("wriie mode on");
	let mut data = String::new();
	io::stdin().read_line(&mut data).expect("can't write into file");

	let data_bytes = data.as_bytes();

	let mut file = OpenOptions::new().write(true).open(file_name).expect("error in writing the file");

	file.write_all(data_bytes).expect("error");
}

pub fn createfile(file_name: String) {
	File::create(file_name).expect("can't create file");
	println!("file created");
}
