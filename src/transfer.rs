use menu;
use comms;
use nix::libc;
use nix::fcntl;
use std;
use std::fs;
use std::ffi::CStr;
use std::ffi::CString;

use pbr;
use pbr::{ProgressBar, Units};
use std::thread;

pub const TRANSFER_CHAR: &'static str = "#";
pub const PRIORITY:u32= 5;

pub fn upload_file(pipe_name:&str,src:&str,opt:&menu::options)/*-> u32*/{
	let metadata = fs::metadata(src);
	let size = metadata.unwrap().len();
	//let src = CString::new(src).unwrap(); // No as ptr - Reusing varname Warning
	//let src_fd = libc::open(src.as_ptr(), fcntl::O_WRONLY.bits());

	let fs = format!("{}", size.clone());
	let sn = format!("/tmp/ssfc{}", unsafe{libc::getpid()});
	let qn = format!("/qsfc{}", unsafe{libc::getpid()});
	println!("{}", fs);
	println!("{}", src);
	comms::send_message(pipe_name, &fs, true);
	comms::send_message(pipe_name, &src, true);
	match opt.method {
		menu::method::PIPES=> send_pipe_file(pipe_name, &src, opt, size.clone()),
		menu::method::QUEUE=> send_queue_file(&qn, &src, opt, size.clone()),
		_ => println!("hola"),
	}
	//return size as u32;
	//comms::send_message(pipe_name,fs,true);
	//comms::send_message(pipe_name,src,true);

	/*match expr {
		menu::method::PIPES => 
	}*/
}

pub fn send_pipe_file(pipe_name:&str,src:&str,opt:&menu::options,filesize:u64) /*->libc::ssize_t*/{
	let mut pb = ProgressBar::new(filesize);//Why it get corrupts after let mute byte?
	pb.set_units(Units::Bytes);
	let pipe_nameOri = pipe_name;
	let srcOri = src;
	let pipe_name = CString::new(pipe_name).unwrap(); // No as ptr - Reusing varname Warning
	let src = CString::new(src).unwrap();
	let (mut transfered, mut chunk, mut wchunk): (u64, libc::ssize_t, libc::ssize_t) = (0,0,0); 
	
	
	//pb.format("╢▌▌░╟");

	let mut src_fd = 0;
	unsafe {
		src_fd = libc::open(src.as_ptr(), fcntl::O_RDONLY.bits());// Open File
		libc::mkfifo(pipe_name.as_ptr(), 0o666); // as_ptr moved here
		//let chunksize = opt.chunksize;
		let chunksize =  if opt.chunksize == 0 {comms::MAX_BUFFER} else {opt.chunksize};
		let fifod = /*libc::*/libc::open(pipe_name.as_ptr(), fcntl::O_WRONLY.bits(),PRIORITY);
		let mut byte = CString::new("").unwrap();
		while transfered<filesize {
			chunk = libc::read(src_fd, byte.as_ptr() as *mut libc::c_void, chunksize as usize);
			if chunk == -1 {
				eprintln!("error reading a byte");
			}
			/*let c_str: &CStr = unsafe { CStr::from_ptr(byte.as_ptr()) };
    		let str_slice: &str = c_str.to_str().unwrap();*/
			//println!("dat1:{}", byte.clone().into_string().unwrap());
			//println!("data2:{:?}", byte);
			wchunk =libc::write(fifod ,byte.as_ptr() as *mut libc::c_void,chunksize as usize);
			if(wchunk != -1){
				pb.set(transfered);
				transfered += chunk as u64;	
			}
			//byte = CString::new("").unwrap();
			//println!("clean:{:?}", byte);
			//test_bar();
			//pb.inc();
			//fprogress_bar(&mut pb,filesize, transfered);
		}
		pb.set(filesize);
		libc::close(src_fd);
		libc::close(fifod);
		//libc::unlink(pipe_name.as_ptr());
	}
	print!("Upload");//Keep the upload status
	pb.finish_print("Ok");
	//return transfered;
}

pub fn send_queue_file(queue:&str,src:&str,opt:&menu::options,filesize:u64) /*->libc::ssize_t*/{
	println!("aja");
	let mut pb = ProgressBar::new(filesize);//Why it get corrupts after let mute byte?
	pb.set_units(Units::Bytes);
	let srcOri = src;
	let queueOri = queue;
	let src = CString::new(src).unwrap();
	let (mut transfered, mut chunk, mut wchunk): (u64, libc::ssize_t, libc::ssize_t) = (0,0,0); 
	
	
	//pb.format("╢▌▌░╟");

	//let mut src_fd = 0;
	let chunksize =  if opt.chunksize == 0 {comms::MAX_BUFFER} else {opt.chunksize};
	//let mut attr:libc::mq_attr = libc::mq_attr{mq_curmsgs:0,mq_flags:0,mq_maxmsg:10,mq_msgsize:chunksize};
	//attr.mq_maxmsg = 10;
	//attr.mq_msgsize = chunksize;
	let mut byte = CString::new("").unwrap();
	println!("{}", queue);
	let mut queue = CString::new(queue).unwrap();

	unsafe {
		let src_fd = libc::open(src.as_ptr(), fcntl::O_RDONLY.bits());// Open File
		let msg_queue:libc::mqd_t = libc::mq_open(queue.as_ptr(), fcntl::O_WRONLY.bits() | fcntl::O_CREAT.bits());
		if msg_queue < 0 {
			println!("No se pudo cargar la msg que")
		}
		//let msg_queue:libc::mqd_t = libc::mq_open(queue.as_ptr(), fcntl::O_WRONLY.bits() | fcntl::O_CREAT.bits(), 0o666, &attr);
		while transfered<filesize {
			//println!("aja2-{}",transfered);
			chunk = libc::read(src_fd, byte.as_ptr() as *mut libc::c_void, chunksize as usize);
			if chunk == -1 {
				eprintln!("error reading a byte");
			}
			let slice = CStr::from_ptr(byte.as_ptr());
			let byte_size = slice.to_bytes().len();
    	//println!("string length: {}", slice.to_bytes().len());
			wchunk =libc::mq_send(msg_queue ,byte.as_ptr(),byte_size,chunksize as u32) as isize;
			if(wchunk != -1){
				pb.set(transfered);
				transfered += chunk as u64;	
			}
		}
		pb.set(filesize);
		libc::close(src_fd);
		//libc::close(fifod);
		//libc::unlink(pipe_name.as_ptr());
	}
	print!("Upload");//Keep the upload status
	pb.finish_print("Ok");
	//return transfered;
}

pub fn receive_pipe_file(pipe_name:&str,piped:i32,opt:&menu::options,filesize:i32) /*->libc::ssize_t*/{
	let mut pb = ProgressBar::new(filesize as u64);//Why it get corrupts after let mute byte?
	pb.set_units(Units::Bytes);
	let pipe_nameOri = pipe_name;
	//let srcOri = src;
	let pipe_name = CString::new(pipe_name).unwrap(); // No as ptr - Reusing varname Warning
	//let src = CString::new(src).unwrap();
	let (mut transfered, mut chunk, mut wchunk): (i32, libc::ssize_t, libc::ssize_t) = (0,0,0); 
	
	
	//pb.format("╢▌▌░╟");

	let mut src_fd = 0;
	unsafe {
		//src_fd = libc::open(src.as_ptr(), fcntl::O_RDONLY.bits());// Open File
		libc::mkfifo(pipe_name.as_ptr(), 0o666); // as_ptr moved here
		//let chunksize = opt.chunksize;
		let mut chunksize =  if opt.chunksize == 0 {comms::MAX_BUFFER} else {opt.chunksize};
		let fifod = /*libc::*/libc::open(pipe_name.as_ptr(), fcntl::O_RDONLY.bits());
		let mut byte = CString::new("").unwrap();
		while transfered<filesize {
			chunk = libc::read(fifod, byte.as_ptr() as *mut libc::c_void, chunksize as usize);
			if (filesize - transfered) > chunksize{
				chunksize = chunksize
			}else{
				chunksize = filesize - transfered;
			}
			if chunk == -1 {
				eprintln!("error reading a byte");
			}
			/*let c_str: &CStr = unsafe { CStr::from_ptr(byte.as_ptr()) };
    		let str_slice: &str = c_str.to_str().unwrap();*/
			//println!("dat1:{}", byte.clone().into_string().unwrap());
			//println!("data2:{:?}", byte);
			wchunk =libc::write(piped ,byte.as_ptr() as *mut libc::c_void,chunksize as usize);
			if(wchunk != -1){
				pb.set(transfered as u64);
				transfered += chunk as i32;	
			}
			//byte = CString::new("").unwrap();
			//println!("clean:{:?}", byte);
			//test_bar();
			//pb.inc();
			//fprogress_bar(&mut pb,filesize, transfered);
		}
		pb.set(filesize as u64);
		//libc::close(src_fd);
		//libc::close(fifod);
		//libc::unlink(pipe_name.as_ptr());
	}
	print!("Upload");//Keep the upload status
	pb.finish_print("Ok");
	//return transfered;
}
/*fn init_progress_bar()->{
	return ProgressBar::new(count);
}*/

fn test_bar(){
	let count=1000;
    let mut pb = ProgressBar::new(count);
    //pb.format("╢▌▌░╟");
    for _ in 0..count {
        pb.inc();
        thread::sleep_ms(200);
    }
    pb.finish_print("done");
}

fn fprogress_bar(pb:&mut pbr::ProgressBar<std::io::Stdout>,filesize:u64,transfered:libc::ssize_t){
	//let count = filesize;
    //let mut pb = ProgressBar::new(count);
    
    pb.inc();
    /*for _ in 0..count {
        pb.inc();
        thread::sleep_ms(200);
    }*/
    
}