#include "transfer.h"

void upload_file(const char *pipe_name, 
				   const char *src, 
				   size_t max_rate, enum method *m) {

	if(max_rate == 0) max_rate = MAX_BUFFER;

	int src_fd, transfered = 0;
	off_t file_size;
	char buffer[max_rate];
	enum method method_value = PIPES;

	/* method_value will decide which method we use in
	 * the transfer by using a switch/case, function will
	 * be split in the future */
	if(m != NULL) method_value = *m;

	src_fd = open(src, O_RDONLY);
	file_size = lseek(src_fd, 0, SEEK_END);
	lseek(src_fd, (off_t) 0, SEEK_SET);

	while(1) {
		int err = read(src_fd, buffer, max_rate);
		/* printf("%d - size: %lu, t: %d\n", err, file_size, transfered); */
		if(err == -1) {
			fprintf(stderr, "Was not able to read this file...\n");
			break;
		}

		if(err == 0) break;

		send_message(pipe_name, buffer, true);

		transfered += err;
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
	int isize = buffer_size("\r%3.2f%% ", percentage, size.ws_col - 11);
	int barsize = size.ws_col - isize;

	char *progress_str = malloc(size.ws_col - isize);
	float progress_chars = ((float)(barsize)/100) * percentage;


	for(int i = 0; i < progress_chars - 1; i++) {
		progress_str[i] = TRANSFER_CHAR;
	}

	fprintf(file, "\r%3.2f%% [%-*s]", percentage, barsize, progress_str);
	fflush(file);

}
