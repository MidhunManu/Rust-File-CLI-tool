use std::env;

mod fileops;

fn main() {

	let args:Vec<String> = env::args().collect();
	let file_name = &args[2];


	if args.len() < 2 {
		std::process::exit(1);
	}	

	match args.get(1) {
		Some(args) => match args.as_str() {
			"read" => fileops::readfile(file_name.to_string()),
			"write" => fileops::writefile(file_name.to_string()),
			"create" => fileops::createfile(file_name.to_string()),
			_ => println!("invalid command"),
		},
		None => println!("no arguments provided"),
	}
}
