//extern crate libc;
extern crate nix;
use nix::libc;
use nix::libc::c_char;
use nix::unistd;//why unistd uses libc instead of wrapper with Result for safety?
use nix::fcntl;

//use std;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
//pub use self::FcntlArg::*;
use std::fs;



pub const SIGNAL:u32= 0;
pub const SENDER:u32= 1;
pub const MAX_TOKENS:u32= 10;
pub const DFT_TRIES:u32= 7;
pub const WAIT_TIME:u32= 10000;
pub const MAX_BUFFER:u32= 4096;
pub const MSG_LS: &'static str = "ls";
pub const MSG_STATUS: &'static str = "status";
pub const MSG_UPLD: &'static str = "upload";
pub const MSG_DONE: &'static str = "done";
pub const MSG_UPLD_E: &'static str = "eupload";
pub const MSG_UPLD_C: &'static str = "cupload";
pub const MSG_DOWNLD: &'static str = "download";
pub const MSG_EXIT: &'static str = "bye";
pub const MSG_ARRIVE: &'static str = "hello";

pub fn send_message(pipe_name:&str,msg:&str,do_unlink:bool){
	//println!("{}", pipe_name);
	//println!("{}", msg);
	let pipe_nameOri = pipe_name;
	let pipe_name = CString::new(pipe_name).unwrap(); // No as ptr - Reusing varname Warning
	unsafe {
	    libc::mkfifo(pipe_name.as_ptr(), 0o666); // as_ptr moved here
	    let fifod = /*libc::*/libc::open(pipe_name.as_ptr(), fcntl::O_WRONLY.bits());
	    //let processed_message = format!("{}w", unsafe {libc::getpid()}); //Format String why
	    let processed_message = format!("{:07}{}", unsafe {libc::getpid()},msg); //Format String
	    println!("{}", processed_message);
	    let pm_size = processed_message.len();
	    let processed_message = CString::new(processed_message).unwrap();
	    libc::write(fifod ,processed_message.as_ptr() as *const libc::c_void,pm_size);
	    libc::close(fifod);
	}
	if do_unlink {
		unsafe {
			libc::unlink(pipe_name.as_ptr());
			//fs::remove_file(pipe_nameOri);
		}
		//unimplemented!();
	}
}

pub fn wait_message(pipe_name:&str,tries:u32) -> [String; 2]{
	let mut count = 0;
	//let byte:c_char = 0;
	let mut byte = CString::new("a").unwrap();
	let mut msg: [String; 2] = Default::default();//["a".to_string(); 2];
	//let mut msg: Vec<String> = vec![String::new(); 2];
	let mut msg_buffer = String::new();
	let filename = CString::new(pipe_name).unwrap(); // No as ptr
	let mut fifod_g = 2;
	unsafe {
		println!("Hei1");
	    libc::mkfifo(filename.as_ptr(), 0o666); // as_ptr moved here
	    println!("Hei2");
	    let fifod = /*libc::*/libc::open(filename.as_ptr(), fcntl::O_RDONLY.bits());
	    println!("Hei3");
	    fifod_g = fifod;
		let mut err = 0; // read return value
		err = libc::read(fifod,byte.as_ptr() as *mut libc::c_void,1);
		println!("Hei4");
		println!("\tOIEAErr:{} tries: {}", err,tries);
		while err > 0 {
			println!("\tOIEBErr:{} tries: {}", err,tries);
			let byte_u8 = CStr::from_ptr(byte.as_ptr()).to_bytes();
			if byte_u8.len() > 0 { //Check if array is empty
				let byte_u8 = byte_u8[0];//get first char?
				msg_buffer.push(byte_u8 as char);
			}			
			err = libc::read(fifod,byte.as_ptr() as *mut libc::c_void,1);
		}
		if err == -1 && tries > 0 {
			libc::usleep(WAIT_TIME);
			libc::close(fifod);
			return wait_message(pipe_name, tries - 1);
		} else if(tries == 0){
			msg[0] = "TIME_OUT".to_string();
			msg[1] = "0".to_string();
			libc::close(fifod);
			return msg;
		}
	}
	//let msg = String::new();
	msg[0] = format!("{}", msg_buffer); //Format String
	msg[1] = format!("{}", msg_buffer); //Format String
	unsafe{
		libc::close(fifod_g);
	}
	return msg;
}