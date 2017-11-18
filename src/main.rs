//#[macro_use]
//extern crate syscall;
//extern crate libc;
extern crate nix;
use nix::libc;
use std::process;
use std::process::Command;
/*use libc::{self, c_char, c_void, c_int, c_long, c_uint, size_t, pid_t, off_t,
           uid_t, gid_t, mode_t};*/
mod menu;
mod comms;

const sfs_path: &'static str = "/tmp/sfs";

fn main() {
	
	let mut opt = menu::MenuOpt::NotValid;
    let mut self_read = String::new();
	let mut self_write = String::new();
    let self_read = format!("/tmp/sfc{}r", unsafe {libc::getpid()});
	let self_write = format!("/tmp/sfc{}w", unsafe {libc::getpid()});
	/*let self_write = format!("/tmp/sfc{}r", unsafe {libc::getpid()});//Lel
	let self_write = format!("/tmp/sfc{}w", unsafe {libc::getpid()});*/
	let test = format!("{:07}", 42);
    println!("Waiting for server...");
    comms::send_message(sfs_path, comms::MSG_ARRIVE, false);
    let res = comms::wait_message(sfs_path, comms::DFT_TRIES);
	println!("{}", test);
	//println!("{}", menu::MenuOpt::NotValid as i32);
    //self_write = "hola".to_string();;
    while opt != menu::MenuOpt::Exit {
    	/*if !Command::new("clear").status().unwrap().success() {//Clears screen an check status-- //TODO Why runs only when checking success?
		    println!("Error clearing screen");
		}*/
    	opt = menu::run_menu();
    	match opt {
    		menu::MenuOpt::ServerLs => {
    			//send_message(self_write, MSG_LS, true);
    			comms::send_message(&self_write,comms::MSG_LS,true);
                let res = comms::wait_message(&self_read,comms::DFT_TRIES);
                /*comms::send_message(&self_write, comms::MSG_LS, true);
                let res = comms::wait_message(sfs_path, comms::DFT_TRIES);*/
                menu::info_screen(&res[0]);
    			/*println!("{}",self_write);
    			println!("{}",comms::MSG_LS);*/
    		},
    		menu::MenuOpt::ServerLs =>{
    			comms::send_message(&self_write,comms::MSG_EXIT,true);
    			process::exit(0);
    		},
    		_ =>{
    			println!("what");
    		},
    	}
    }

}

fn wait_for_server(){
	println!("Rust Client v0.0.1 created by GG\n");
	comms::send_message(sfs_path, comms::MSG_ARRIVE, false);
	println!("Waiting for Server...");
}