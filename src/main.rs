//#[macro_use]
//extern crate syscall;
//extern crate libc;
extern crate nix;
extern crate pbr;

use nix::libc;
use nix::sys::signal;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
/*use libc::{self, c_char, c_void, c_int, c_long, c_uint, size_t, pid_t, off_t,
           uid_t, gid_t, mode_t};*/
mod menu;
mod comms;
mod status;
mod transfer;

const sfs_path: &'static str = "/tmp/sfs";

static mut cancelInterrupt:bool = false;
static mut connectedServer:bool = false;

fn main() {
	//-----------------Manejo de senal de interrupcion
	set_interrupt_handler();
	
  	//------------------Inicio de codigo de cliente
	let mut opt = menu::MenuOpt::NotValid;
	let mut self_read = String::new();
	let mut self_write = String::new();

	//let mut status:status::client_status;
	let tmpDir  = status::directory{files:Vec::new(),file_count:0};
	let mut status = status::client_status{
		dir:"".to_string(),
		server_dir:"".to_string(),
		current_dir:tmpDir,
		opts:menu::get_default_opts(),
		pid:0,
		server_pid:0,
	};

	status.opts = menu::get_default_opts();
	status.dir = (status::DEFAULT_DIR).to_string();
	status.current_dir = status::get_dir_contents(&(status.dir));
	let self_read = format!("/tmp/sfc{}r", unsafe {libc::getpid()});
	let self_write = format!("/tmp/sfc{}w", unsafe {libc::getpid()});
	wait_for_server();
	unsafe{connectedServer=true};
	comms::send_message(sfs_path, comms::MSG_ARRIVE, false);
	let res = comms::wait_message(sfs_path, comms::DFT_TRIES);
    let fixServerPid: String = res[comms::SENDER].chars().skip(0).take(7).collect();
    println!("{}", fixServerPid);
	//println!("{}", res[comms::SENDER]);
	let casted_server_pid: i32 = match fixServerPid.trim().parse() {
        Ok(num) => {num},
        Err(_) => {
        	println!("Error, data invalida");
        	0
        	//break; /*continue*/
        },//continue will keep reading same value? Maybe let should be mut?
    };
    println!("{}", casted_server_pid);
	status.server_pid = casted_server_pid;//res[comms::SENDER].parse::<i32>().unwrap();
	status.server_dir = res[comms::SIGNAL].clone();
	status.opts.chunksize = 0;
	//println!("{}", menu::MenuOpt::NotValid as i32);
	//self_write = "hola".to_string();;
	while opt != menu::MenuOpt::Exit {
		/*if !Command::new("clear").status().unwrap().success() {//Clears screen an check status-- //TODO Why runs only when checking success?
		  println!("Error clearing screen");
		  }*/
		opt = menu::run_menu();
		match opt {
			menu::MenuOpt::ServerLs => {
				comms::send_message(&self_write,comms::MSG_LS,true);
				let res = comms::wait_message(&self_read,comms::DFT_TRIES);
				menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::ServerState => {
				comms::send_message(&self_write,comms::MSG_STATUS,true);
				let res = comms::wait_message(&self_read,comms::DFT_TRIES);
				menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::UpldFile => {
				comms::send_message(&self_write,comms::MSG_UPLD,true);
				transfer::upload_file(&self_write,&status.current_dir.files[3],&status.opts);
				println!("Press enter to continue...");
				//let res = comms::wait_message(&self_read,comms::DFT_TRIES);
				//menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::ClientLs => {
				let dir_status = status::sprint_dir_status(&status);
				menu::info_screen(&dir_status);
				/*println!("{}",self_write);
				  println!("{}",comms::MSG_LS);*/
			},
			menu::MenuOpt::Exit =>{
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
	println!("Waiting for Server...");
	//comms::send_message(sfs_path, comms::MSG_ARRIVE, false);	
}

//--------------------------------------------Manejo de senal de interrupcion
fn set_interrupt_handler(){
	let handler_sig = signal::SigHandler::Handler(handle_sigint);
	let flags = signal::SA_ONSTACK | signal::SA_RESTART | signal::SA_SIGINFO;

    let mut mask = signal::SigSet::empty();
    mask.add(signal::SIGUSR1);
	let sig_action = signal::SigAction::new(handler_sig,flags,mask);
	unsafe {
		signal::sigaction(signal::SIGINT, &sig_action);	
	}
}

extern fn handle_sigint(_:i32) {
	if(unsafe{connectedServer}==false){
		println!("\rInterrupted Signal-No server Found...");
		unsafe{libc::exit(0)};
	}
	if(unsafe{cancelInterrupt} == false){
		println!("\rSelect 8 to exit the client correctly");
		print!("{}",menu::PROMPT);
		io::stdout().flush().unwrap();
		unsafe{cancelInterrupt= true};
	}else{
		println!("\rInterrupted Signal called Twice. Exiting...");
		let self_write = format!("/tmp/sfc{}w", unsafe {libc::getpid()});
		comms::send_message(&self_write,comms::MSG_EXIT,true);
		unsafe{libc::exit(0)};
	}
  	//panic!();
}