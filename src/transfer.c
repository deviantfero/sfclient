#include "transfer.h"

int upload_file(const char *pipe_name, 
				   char *src, 
				   int chunksize, enum method *m) {

	int src_fd;
	off_t filesize;
	char fs[MAX_BUFFER];
	char cs[MAX_BUFFER];
	char sn[MAX_BUFFER];
	/* enum method method_value = PIPES; */

	if(chunksize == 0) chunksize = MAX_BUFFER;

	/* method_value will decide which method we use in
	 * the transfer by using a switch/case, function will
	 * be split in the future */
	/* if(m != NULL) method_value = *m; */

	src_fd = open(src, O_RDONLY);
	filesize = lseek(src_fd, 0, SEEK_END);
	lseek(src_fd, (off_t) 0, SEEK_SET);

	snprintf(fs, MAX_BUFFER, "%ld", filesize);
	snprintf(cs, MAX_BUFFER, "%d", chunksize);
	snprintf(sn, MAX_BUFFER, "/tmp/ssfc%d", getpid());

	send_message(pipe_name, cs, true);
	send_message(pipe_name, fs, true);
	send_message(pipe_name, src, true);

	switch(*m) {
		case PIPES: return (send_pipe_file(pipe_name, src_fd, chunksize, filesize) == filesize);
		case SOCKETS: return (send_sock_file(sn, src_fd, chunksize, filesize) == filesize);
		default: return filesize;
	}
	return (send_pipe_file(pipe_name, src_fd, chunksize, filesize) == filesize);
}

int send_pipe_file(const char *pipe_name, int src_fd, int chunksize, size_t filesize) {
	mkfifo(pipe_name, 0666);
	int fifod = open(pipe_name, O_WRONLY), transfered = 0, chunk = 0, wchunk = 0;
	char byte[chunksize + 1];

	for(int i = 0; (size_t)transfered < filesize; i++) {
		if((chunk = read(src_fd, byte, chunksize)) == -1) fprintf(stderr, "error reading a byte");
		wchunk = write(fifod, byte, chunksize);
		if(wchunk != -1)
			transfered += chunk;
		fprogress_bar(stdout, filesize, transfered);
	}

	close(src_fd);
	close(fifod);
	/* unlink(pipe_name); */

	return transfered;
}

int send_sock_file(const char *sock_name, int src_fd, int chunksize, size_t filesize) {
	int csock = make_named_sock(sock_name), ssock;
	char buffer[chunksize + 1];
	size_t transfered = 0, chunk = 0, wchunk = 0;

	if(listen(csock, 10) < 0) {
		fprintf(stderr, "failed to listen %s\n", strerror(errno));
		return 0;
	}

	ssock = accept(csock, (struct sockaddr*)NULL, NULL);
	for(int i = 0; (size_t)transfered < filesize; i++) {
		if((chunk = read(src_fd, buffer, chunksize)) == -1) fprintf(stderr, "error reading a byte");
		wchunk = write(ssock, buffer, chunksize);
		if(wchunk != -1)
			transfered += chunk;
		fprogress_bar(stdout, filesize, transfered);
	}
	close(ssock);

	return transfered;
}

int make_named_sock(const char *sock_name){
	struct sockaddr_un name;
	int sock;
	size_t size;

	if((sock = socket(PF_LOCAL, SOCK_STREAM, 0)) < 0) {
		fprintf(stderr, "socket not created\n");
		return 0;
	}

	name.sun_family = AF_LOCAL;
	strncpy(name.sun_path, sock_name, sizeof(name.sun_path));
	name.sun_path[sizeof(name.sun_path) - 1] = '\0';

	size = SUN_LEN(&name);
	size = (offsetof (struct sockaddr_un, sun_path) + strlen(name.sun_path));

	if(bind(sock, (struct sockaddr *)&name, size) < 0) {
		fprintf(stderr, "socket bind fail\n");
		return 0;
	}

	return sock;
}


int receive_pipe_file(const char *pipe_name, int piped, int chunksize, size_t filesize) {
	int fifod = open(pipe_name, O_RDONLY), err, wchunk;
	size_t count = 0;
	char byte[chunksize + 1];
	memset(byte, 0, chunksize + 1);

	while((err = read(fifod, byte, chunksize)) > 0 && count < filesize) {
		/* making sure not to insert trash at the end of file */
		chunksize = ((size_t)(filesize - count) > (size_t)chunksize) ? 
					(size_t)chunksize : (size_t)(filesize - count);

		if(err != -1 && (wchunk = write(piped, byte, chunksize)) != -1) {
			count += wchunk;
		}
		fprogress_bar(stdout, filesize, count);
		memset(byte, 0, chunksize + 1);
	}
	return count;
}


void fprogress_bar(FILE *file, off_t file_size, size_t transfered) {
	float percentage = ((float)100/file_size) * transfered;
	struct winsize size;
	ioctl(STDOUT_FILENO, TIOCGWINSZ, &size);

	int isize = buffer_size("\r%3.2f%% %d bytes", percentage, transfered);
	int barsize = size.ws_col - isize;

	char *progress_str = malloc(size.ws_col - isize);
	memset(progress_str, 0, size.ws_col - isize);
	float progress_chars = ((float)(barsize)/100) * percentage;


	for(int i = 0; i < progress_chars - 1; i++) {
		progress_str[i] = TRANSFER_CHAR;
	}

	fprintf(file, "\r%3.2f%% %ld bytes [%-*s]", percentage, transfered, barsize - 1, progress_str);
	fflush(file);
}