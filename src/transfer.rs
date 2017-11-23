use menu;
use comms;
use nix::libc;
use nix::fcntl;
use std;
use std::fs;
use std::ffi::CString;

use pbr;
use pbr::ProgressBar;
use std::thread;

pub const TRANSFER_CHAR: &'static str = "#";

pub fn upload_file(pipe_name:&str,src:&str,opt:&menu::options)-> u32{
	let metadata = fs::metadata(src);
	let size = metadata.unwrap().len();
	//let src = CString::new(src).unwrap(); // No as ptr - Reusing varname Warning
	//let src_fd = libc::open(src.as_ptr(), fcntl::O_WRONLY.bits());

	let fs = format!("{}", size.clone());
	let sn = format!("/tmp/ssfc{}", unsafe{libc::getpid()});
	println!("{}", fs);
	println!("{}", src);
	match opt.method {
		PIPES => send_pipe_file(pipe_name, &src, opt, size.clone()) as u32,
		_ => size as u32,
	}
	//return size as u32;
	//comms::send_message(pipe_name,fs,true);
	//comms::send_message(pipe_name,src,true);

	/*match expr {
		menu::method::PIPES => 
	}*/
}

pub fn send_pipe_file(pipe_name:&str,src:&str,opt:&menu::options,filesize:u64) ->libc::ssize_t{
	let pipe_nameOri = pipe_name;
	let srcOri = src;
	let pipe_name = CString::new(pipe_name).unwrap(); // No as ptr - Reusing varname Warning
	let src = CString::new(src).unwrap();
	let (mut transfered, mut chunk, mut wchunk): (libc::ssize_t, libc::ssize_t, libc::ssize_t) = (0,0,0); 
	let mut byte = CString::new("a").unwrap();
	let mut pb = ProgressBar::new(filesize);
	pb.format("╢▌▌░╟");
	let mut src_fd = 0;
	unsafe {
		src_fd = libc::open(src.as_ptr(), fcntl::O_RDONLY.bits());// Open File
		libc::mkfifo(pipe_name.as_ptr(), 0o666); // as_ptr moved here
		//let chunksize = opt.chunksize;
		let chunksize =  if opt.chunksize == 0 {comms::MAX_BUFFER} else {opt.chunksize};
		let fifod = /*libc::*/libc::open(pipe_name.as_ptr(), fcntl::O_WRONLY.bits());
		for i in 0..filesize{
			chunk = libc::read(src_fd, byte.as_ptr() as *mut libc::c_void, chunksize as usize);
			if chunk == -1 {
				eprintln!("error reading a byte");
			}
			wchunk =libc::write(fifod ,byte.as_ptr() as *mut libc::c_void,chunksize as usize);
			if(wchunk != -1){
				transfered += chunk;	
			}
			fprogress_bar(&mut pb,filesize, transfered);
		}
		libc::close(src_fd);
		libc::close(fifod);
	}
	pb.finish_print("done");
	return transfered;
}

/*fn init_progress_bar()->{
	return ProgressBar::new(count);
}*/

fn fprogress_bar(pb:&mut pbr::ProgressBar<std::io::Stdout>,filesize:u64,transfered:libc::ssize_t){
	//let count = filesize;
    //let mut pb = ProgressBar::new(count);
    
    pb.set(filesize);
    /*for _ in 0..count {
        pb.inc();
        thread::sleep_ms(200);
    }*/
    
}