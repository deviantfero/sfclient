// An attribute to hide warnings for unused code.
#![allow(dead_code)]
//------------------Header---------------
//extern crate core;

use std::io;
use std::io::Write;// <--- bring flush() into scope
//use self::core::num::FromStrRadixHelper;

const BUFFER_MAX:i32= 100;
pub const PROMPT: &'static str = "Enter a number: ";

//#[derive(Debug)]
#[derive(PartialEq, Eq, Clone)]
pub enum MenuOpt {
	NotValid, 
	ServerLs, 
	ServerState, 
	UpldFile, 
	DwndFile, 
	//ToggleEncryption,
	//ToggleCompression,
	ClientLs,
	SetMode,
	Exit,
}

impl MenuOpt{
	fn from_u32(val:u32) ->MenuOpt{
		match val {
			0 => return MenuOpt::NotValid,
			1 => return MenuOpt::ServerLs,
			2 => return MenuOpt::ServerState,
			3 => return MenuOpt::UpldFile,
			4 => return MenuOpt::DwndFile,
			//5 => return MenuOpt::ToggleEncryption,
			//6 => return MenuOpt::ToggleCompression,
			5 => return MenuOpt::ClientLs,
			6 => return MenuOpt::SetMode,
			7 => return MenuOpt::Exit,
			_ => return MenuOpt::NotValid,
		}
	}
}
#[derive(Clone)]
pub struct options {
	encrypt:bool,
	compress:bool,
	pub chunksize:i32,
	pub method:method,
}
#[derive(Clone)]
pub enum method {
	PIPES,
	QUEUE,
	//SOCKETS,
}

impl method{
	fn from_u32(val:i32) ->method{
		match val {
			0 => return method::PIPES,
			1 => return method::QUEUE,
			_ => return method::PIPES,
		}
	}
}

pub fn get_method_name(m:method) -> &'static str{
	match m {
		method::PIPES => return "pipes",
		method::QUEUE => return "queue",
		//method::SOCKETS => expr,
		_ => return "unknown",
	}
}

pub fn method_menu() ->method{
	let mut read_option:String = String::new();
	let mut read_option_int_g:i32 =0;
	while {
		for i in 0..2 {
			println!("[{}] {}", i , get_method_name(method::from_u32(i)));
		}
		print!("\nEnter a method number:");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut read_option)
            .expect("Failed to read line");
		let read_option_int: i32 = match read_option.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
            	println!("Error, ingrese numeros");
            	2
            	//break; /*continue*/
            },//continue will keep reading same value? Maybe let should be mut?
        };
        read_option_int_g = read_option_int;
        read_option_int < 0 || read_option_int > 2        
	} {};

	return method::from_u32(read_option_int_g);
}

//------------------Body------------------
pub fn print_menu() {
	let encryption_entry = "[off]".to_string(); //Placeholder
	let compression_entry = "[off]".to_string(); //Placeholder
    println!("\t1. See server content");
	println!("\t2. See server status");
	println!("\t3. Upload a file");
	println!("\t4. Download a file");
	//println!("\t5. Encrypt transfer {}", encryption_entry);
	//println!("\t6. Compress transfer {}", compression_entry);
	println!("\t5. See client content");
	println!("\t6. Set transfer method");
	println!("\t7. Exit");
	print!("{}",PROMPT);
	io::stdout().flush().unwrap();//<-- unwrap gets rid of Result
	//println!("{}", MenuOpt::NotValid as i32);
}

pub fn run_menu() -> MenuOpt{
	//let opt:MenuOpt;
	let mut read_option = String::new();
	let mut read_option_int_g= MenuOpt::NotValid;
	loop {
		print_menu();
		
        io::stdin().read_line(&mut read_option)
            .expect("Failed to read line");
		let read_option_int: u32 = match read_option.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
            	println!("Error, ingrese numeros");
            	read_option_int_g = MenuOpt::NotValid;
            	break; /*continue*/
            },//continue will keep reading same value? Maybe let should be mut?
        };
        read_option_int_g = MenuOpt::from_u32(read_option_int);
        break;
        /*if read_option_int == MenuOpt::Exit as u32 {
        	break;
        }*/
	}
	return read_option_int_g;
	//println!("{}", MenuOpt::NotValid as i32);
}

pub fn choose_file(dir_status:&str, file_count:u32) -> MenuOpt{
	let mut tmp = String::new();
	let mut opt= MenuOpt::NotValid;
	while{
		print!("{}\nInput a file number:", dir_status);
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut tmp)
            .expect("Failed to read line");
     	let read_option_int: u32 = match tmp.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
            	println!("Error, ingrese numeros");
            	MenuOpt::NotValid as u32
            	//break; /*continue*/	
            },//continue will keep reading same value? Maybe let should be mut?
        };
        opt = MenuOpt::from_u32(read_option_int);
        if opt == MenuOpt::NotValid {
        	println!("{}", "Enter a valid option...");
        }
        opt == MenuOpt::NotValid
	} {}// im a do- while
	return opt;
}

pub fn info_screen(info:&str){
	println!("{}", info);
}

pub fn get_default_opts() -> options{
	return options{
		encrypt:false,
		compress:false,
		method:method::PIPES,
		chunksize:0,
	}
}