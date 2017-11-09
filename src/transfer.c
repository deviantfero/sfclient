#include "transfer.h"

void upload_file(const char *pipe_name, 
				   const char *read_name,
				   char *src, 
				   size_t max_rate, enum method *m) {

	if(max_rate == 0) max_rate = MAX_BUFFER;

	int src_fd, transfered = 0;
	off_t file_size;
	char buffer[max_rate];
	char chunk_size[MAX_BUFFER];
	enum method method_value = PIPES;

	/* method_value will decide which method we use in
	 * the transfer by using a switch/case, function will
	 * be split in the future */
	if(m != NULL) method_value = *m;

	src_fd = open(src, O_RDONLY);
	file_size = lseek(src_fd, 0, SEEK_END);
	lseek(src_fd, (off_t) 0, SEEK_SET);
	snprintf(chunk_size, MAX_BUFFER, "%ld", max_rate);

	send_message(pipe_name, chunk_size, true);
	send_message(pipe_name, src, true);

	while(1) {
		/* cleaning buffer */
		memset(buffer, 0, max_rate);
		int err = read(src_fd, buffer, max_rate);
		if(err == -1) {
			fprintf(stderr, "Was not able to read this file...\n");
			break;
		}

		transfered += err;
		if(err == 0 || transfered > file_size) break;

		/* printf("strlen: %ld\n", strlen(buffer)); */
		send_message(pipe_name, buffer, true);
		/* wait for server to receive chunk */
		wait_message(read_name, DFT_TRIES);

		/* fprintf(stdout, buffer); */
		fprogress_bar(stdout, file_size, transfered);
	}

	send_message(pipe_name, MSG_DONE, true);
	close(src_fd);
}

void fprogress_bar(FILE *file, off_t file_size, size_t transfered) {
	float percentage = ((float)100/file_size) * transfered;
	struct winsize size;
	ioctl(STDOUT_FILENO, TIOCGWINSZ, &size);
	int isize = buffer_size("\r%3.2f%% %d bytes", percentage, transfered);
	int barsize = size.ws_col - isize;

	char *progress_str = malloc(size.ws_col - isize);
	float progress_chars = ((float)(barsize)/100) * percentage;


	for(int i = 0; i < progress_chars - 1; i++) {
		progress_str[i] = TRANSFER_CHAR;
	}

	fprintf(file, "\r%3.2f%% %ld bytes [%-*s]", percentage, transfered, barsize - 1, progress_str);
	fflush(file);

}
