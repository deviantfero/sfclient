#include "comms.h"

/* opens pipe in Write only mode, 
 * this should freeze the program 
 * until message has been read */
void send_message(const char *pipe_name, char *msg, bool do_unlink) {
	mkfifo(pipe_name, 0666);

	int fifod = open(pipe_name, O_WRONLY);
	char *processed_message = malloc(MAX_BUFFER);

	snprintf(processed_message, MAX_BUFFER, "%s;%d", msg, getpid());
	write(fifod, processed_message, MAX_BUFFER);
	close(fifod);

	if(do_unlink) unlink(pipe_name);
}

char *wait_message(const char *pipe_name) {
	char* msg_buffer = malloc(MAX_BUFFER);

	int fifod = open(pipe_name, O_RDONLY);
	read(fifod, msg_buffer, MAX_BUFFER);
	close(fifod);

	return msg_buffer;
}

