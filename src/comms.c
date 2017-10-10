#include "comms.h"

/* opens pipe in Write only mode, 
 * this should freeze the program 
 * until message has been read */
void send_message(const char *pipe_name, char *msg, bool do_unlink) {
	mkfifo(pipe_name, 0666);

	int fifod = open(pipe_name, O_WRONLY);
	char *processed_message = malloc(sizeof(msg) + 10);

	snprintf(processed_message, sizeof(msg + 10), "%s;%d\n", msg, getpid());
	write(fifod, processed_message, sizeof(processed_message));
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

