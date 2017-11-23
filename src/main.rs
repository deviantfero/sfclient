//#[macro_use]
//extern crate syscall;
//extern crate libc;
extern crate nix;
extern crate pbr;
extern crate walkdir;

use nix::libc;
use nix::fcntl;
use nix::sys::signal;
use std::io;
use std::io::Write;
use std::ffi::CString;
use std::process;
use std::process::Command;
/*use libc::{self, c_char, c_void, c_int, c_long, c_uint, size_t, pid_t, off_t,
           uid_t, gid_t, mode_t};*/
mod menu;
mod comms;
mod status;
mod transfer;

use menu::MenuOpt;

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
	let mut res:[String; 2] = Default::default();;

	let mut server_files;
	//let mut server_file_amount;

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
	let file_menu = status::sprint_dir_status(&status); 
	let self_read = format!("/tmp/sfc{}r", unsafe {libc::getpid()});
	let self_write = format!("/tmp/sfc{}w", unsafe {libc::getpid()});
	wait_for_server();
	unsafe{connectedServer=true};
	comms::send_message(sfs_path, comms::MSG_ARRIVE, false);
	res = comms::wait_message(sfs_path, comms::DFT_TRIES);
    let fixServerPid: String = res[comms::SENDER].chars().skip(0).take(7).collect();
    println!("kha1{}", res[comms::SENDER].trim());
	//println!("{}", res[comms::SENDER]);
	let casted_server_pid: i32 = match fixServerPid.trim().parse() {
        Ok(num) => {num},
        Err(_) => {
        	println!("Error, data invalida");
        	0
        	//break; /*continue*/
        },//continue will keep reading same value? Maybe let should be mut?
    };
    println!("kha2{}", casted_server_pid);
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
				res = comms::wait_message(&self_read,comms::DFT_TRIES);
				menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::ServerState => {
				comms::send_message(&self_write,comms::MSG_STATUS,true);
				res = comms::wait_message(&self_read,comms::DFT_TRIES);
				menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::UpldFile => {
				opt = menu::choose_file(&file_menu,status.current_dir.file_count);
				println!("{}",opt.clone() as u32);
				comms::send_message(&self_write,comms::MSG_UPLD,true);
				transfer::upload_file(&self_write,&status.current_dir.files[opt.clone() as usize],&status.opts);
				io::stdout().flush();
				println!("Press enter to continue...");
				//let res = comms::wait_message(&self_read,comms::DFT_TRIES);
				//menu::info_screen(&res[comms::SIGNAL]);
			},
			menu::MenuOpt::DwndFile => {
				let mut total:libc::ssize_t = 0;
				comms::send_message(&self_write,comms::MSG_DOWNLD,true);
				/* file list in server */
				res = comms::wait_message(&self_read,comms::DFT_TRIES);
				//println!("Mira1:{}", res[comms::SIGNAL]);
				//println!("Mira2:{}", res[comms::SENDER]);
				server_files = res[comms::SIGNAL].clone();

				/* wait file amount in server */
				res = comms::wait_message(&self_read,comms::DFT_TRIES);
				
				let server_file_amount: u32 = match res[comms::SIGNAL].trim().parse() {
		            Ok(num) => {num},
		            Err(_) => {
		            	println!("Error, ingrese numeros");
		            	break; /*continue*/
		            },//continue will keep reading same value? Maybe let should be mut?
		        };
				/* send file selection to server */
				println!("Server PID:{}", res[comms::SENDER]);
				opt = menu::choose_file(&server_files,server_file_amount);
				println!("{}", opt.clone() as u32);
				comms::send_message(&self_write,&format!("{}",opt.clone() as u32),true);
				
				/* receive filename, filesize */
				res = comms::wait_message(&self_read,comms::DFT_TRIES);
				
				let filesize: i32 = match res[comms::SIGNAL].trim().parse() {
		            Ok(num) => {num},
		            Err(_) => {
		            	println!("Error, ingrese numeros");
		            	break; /*continue*/
		            },//continue will keep reading same value? Maybe let should be mut?
		        };
				res = comms::wait_message(&self_read,comms::DFT_TRIES);

				let mut ressignal = CString::new(res[comms::SIGNAL].clone()).unwrap();
				let nfd = unsafe{libc::open(ressignal.as_ptr(), fcntl::O_WRONLY.bits() | fcntl::O_CREAT.bits() | fcntl::O_TRUNC.bits(),0o644)};// Open File
				//int nfd = open(res[comms::SIGNAL], O_WRONLY | O_CREAT | O_TRUNC, 0644);
				println!("downloading {} from ({})...", res[comms::SIGNAL], res[comms::SENDER]);
				transfer::receive_pipe_file(&self_read, nfd, &status.opts, filesize);
				/*switch(status->opts->method) {
					case PIPES: 
						while((total += receive_pipe_file(self_read, nfd, status->opts, filesize)) < filesize);
						break;
					case SOCKETS:
						while((total += receive_sock_file(self_socket, nfd, status->opts, filesize)) < filesize);
					case QUEUE:
						while((total += receive_queue_file(self_queue, nfd, status->opts, filesize)) < filesize);
						break;
					default: break;
				}*/

				/* decompress file */
				//if(status->opts->compress) inflate_file(res[comms::SIGNAL], true);

				//status->current_dir = status::get_dir_contents(&(status.dir));

				println!("Press enter to continue...");
			},
			menu::MenuOpt::ClientLs => {
				status.current_dir = status::get_dir_contents(&(status.dir));//test
				let dir_status = status::sprint_dir_status(&status);
				menu::info_screen(&dir_status);
				/*println!("{}",self_write);
				  println!("{}",comms::MSG_LS);*/
			},
			menu::MenuOpt::SetMode => {
				status.opts.method = menu::method_menu();//test
				comms::send_message(&self_write,comms::MSG_METHOD,true);
				comms::send_message(&self_write,&format!("{}",status.clone().opts.method  as u32),true);
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