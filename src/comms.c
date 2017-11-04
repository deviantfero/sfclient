#include "comms.h"

/* opens pipe in Write only mode, 
 * this should freeze the program 
 * until message has been read */
void send_message(const char *pipe_name, char *msg, bool do_unlink) {
	mkfifo(pipe_name, 0666);

	int fifod = open(pipe_name, O_WRONLY);
	int pm_size = buffer_size("%s;%d", msg, getpid());
	char *processed_message = malloc(pm_size);

	snprintf(processed_message, pm_size, "%s;%d", msg, getpid());
	write(fifod, processed_message, pm_size);
	close(fifod);

	if(do_unlink) unlink(pipe_name);
}

char **wait_message(const char *pipe_name) {
	char *msg_buffer = malloc(1);
	char byte = 0;
	char **msg = malloc(sizeof(char*) * MAX_TOKENS);
	char *token;
	int count = 0;

	if(msg_buffer == NULL || msg == NULL) {
		return NULL;
	}

	int fifod = open(pipe_name, O_RDONLY);

	while(read(fifod, &byte, 1) == 1) {
		msg_buffer = realloc(msg_buffer, count  + 1);
		msg_buffer[count] = byte;
		count++;
	}

	for(int i = 0; (token = strsep(&msg_buffer, ";")) != NULL && i < MAX_TOKENS; i++)
		msg[i] = token;

	close(fifod);
	free(msg_buffer);
	return msg;
}

