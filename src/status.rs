use std::fs;
use menu;

pub const DEFAULT_DIR: &'static str = "./";

pub struct directory {
	pub files:Vec<String>,
	pub file_count:u32,
}

pub struct client_status {
	pub dir:String,
	pub server_dir:String,
	pub current_dir:directory,
	pub opts:menu::options,
	pub pid:i32,
	pub server_pid:i32,
}

pub fn sprint_dir_status(s:&client_status)->String{
	let mut str = String::new();
	for path in s.current_dir.files.iter() {
		str = format!("{}{}\n", str,path); //Format String
	}
	return str;
}

pub fn get_dir_contents(dir:&str) -> directory{
	let paths = fs::read_dir(dir).unwrap();
	let mut current_dir = directory{files:Vec::new(),file_count:0};
    for path in paths {
    	let readedFile = format!("{}", path.unwrap().path().display()); //Format String
    	current_dir.files.push(readedFile);
    	current_dir.file_count = current_dir.file_count + 1;
        //println!("Name: {}", current_dir.files.last().unwrap());//->Last puts in some(), unwrap clones the valuee inside it
    }
    return current_dir;
}