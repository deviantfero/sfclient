use std::fs;
use menu;

use walkdir::{DirEntry, WalkDir};

pub const DEFAULT_DIR: &'static str = "./";

#[derive(Clone)]
pub struct directory {
	pub files:Vec<String>,
	pub file_count:u32,
}
 #[derive(Clone)]
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
	let mut index = 0;
	for path in s.current_dir.files.iter() {
		str = format!("{}[{}] {}\n", str,index,path); //Format String
		index+=1;
	}
	return str;
}

pub fn get_dir_contents(dir:&str) -> directory{
	let paths = fs::read_dir(dir).unwrap();
	let mut current_dir = directory{files:Vec::new(),file_count:0};
	//let md = metadata(".").unwrap();
    //println!("is dir: {}", md.is_dir());
    for path in paths {
    	let readedFile = format!("{}", path.unwrap().path().display()); //Format String
    	let readedFile:String = readedFile.chars().skip(2).collect();
    	if(fs::metadata(readedFile.clone()).unwrap().is_file()){
	    	current_dir.files.push(readedFile.clone());
	    	current_dir.file_count = current_dir.file_count + 1;
	    	//println!("{}", readedFile);
    	}
    	
        //println!("Name: {}", current_dir.files.last().unwrap());//->Last puts in some(), unwrap clones the valuee inside it
    }
    return current_dir;
}


pub fn get_dir_contentsOld2(dir:&str) -> directory{
	let paths = fs::read_dir(dir).unwrap();
	let walker = WalkDir::new(dir).into_iter();
	
	let mut current_dir = directory{files:Vec::new(),file_count:0};
	for entry in walker.filter_entry(|e| !is_hidden(e)) {
		let readedFile = format!("{}", entry.unwrap().path().display()); //Format String
    	current_dir.files.push(readedFile.clone());
    	current_dir.file_count = current_dir.file_count + 1;
    	println!("{}", readedFile);
	    //println!("{}", entry.unwrap().path().display());
	}
    /*for path in paths {
    	let readedFile = format!("{}", path.unwrap().path().display()); //Format String
    	current_dir.files.push(readedFile);
    	current_dir.file_count = current_dir.file_count + 1;
        //println!("Name: {}", current_dir.files.last().unwrap());//->Last puts in some(), unwrap clones the valuee inside it
    }*/
    return current_dir;
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn get_dir_contentsOld(dir:&str) -> directory{
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