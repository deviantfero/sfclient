#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <limits.h>
#include <unistd.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/ioctl.h>

#include "comms.h"
#include "menu.h"
#include "status.h"
#include "transfer.h"


struct client_status *status;
char *self_write, *self_read;

void exitHandler(int sig) {
	send_message(self_write, MSG_EXIT, true);
	exit(0);
}

int main(void) {
	const char* sfs_path = "/tmp/sfs";
	/* two pipes per client for an individual thread
	 * in the server, one for reading, one for writing. */
	self_read = malloc(MAX_BUFFER); 
	self_write = malloc(MAX_BUFFER);
	char *self_socket = malloc(MAX_BUFFER);
	char *self_queue = malloc(MAX_BUFFER);
	char **res, *dir_status;
	char *server_files;
	char* tmp_buffer; //multiple uses 
	int server_file_amount;

	status = malloc(sizeof(struct client_status*));

	snprintf(self_read, MAX_BUFFER, "/tmp/sfc%dr", getpid());
	snprintf(self_write, MAX_BUFFER, "/tmp/sfc%dw", getpid());
	snprintf(self_socket, MAX_BUFFER, "/tmp/ssfc%d", getpid());
	snprintf(self_queue, MAX_BUFFER, "/qsfc%d", getpid());

	status->opts = get_default_opts();
	status->dir = DEFAULT_DIR;
	status->current_dir = get_dir_contents(status->dir);
	char *file_menu = sprint_dir_status(status); 
	enum menu_opt opt = SERVER_LS;

	/* shake hands */
	puts("Waiting for server...");
	send_message(sfs_path, MSG_ARRIVE, false);
	res = wait_message(sfs_path, DFT_TRIES);
	status->server_pid = atoi(res[SENDER]);
	status->server_dir = res[SIGNAL];
	status->opts->chunksize = 0;

	signal(SIGINT, exitHandler);

	while(opt != EXIT) {
		system("clear");
		opt = run_menu(status->opts);
		switch(opt) {
			case SERVER_LS:
				send_message(self_write, MSG_LS, true);
				res = wait_message(self_read, DFT_TRIES);
				info_screen(res[SIGNAL]);
				break;
			case SERVER_STATE:
				send_message(self_write, MSG_STATUS, true);
				res = wait_message(self_read, DFT_TRIES);
				info_screen(res[SIGNAL]);
				break;
			case UPLD_FILE:
				opt = choose_file(file_menu, status->current_dir->file_count);
				send_message(self_write, MSG_UPLD, true);
				upload_file(self_write, 
							status->current_dir->files[opt], 
							status->opts);
				fflush(stdout);
				fprintf(stdout, "\nPress enter to continue...");
				while(getchar() != 10);
				break;
			case DWNLD_FILE: 
			{ 
				ssize_t total = 0;
				send_message(self_write, MSG_DOWNLD, true);
				/* file list in server */
				res = wait_message(self_read, DFT_TRIES);
				server_files = res[SIGNAL];

				/* wait file amount in server */
				res = wait_message(self_read, DFT_TRIES);
				server_file_amount = atoi(res[SIGNAL]);

				/* send file selection to server */
				opt = choose_file(server_files, server_file_amount);
				tmp_buffer = malloc(buffer_size("%d", opt));
				snprintf(tmp_buffer, buffer_size("%d", opt), "%d", opt);
				send_message(self_write, tmp_buffer, true);
				
				/* receive filename, filesize */
				res = wait_message(self_read, DFT_TRIES);
				int filesize = atoi(res[SIGNAL]);
				res = wait_message(self_read, DFT_TRIES);

				int nfd = open(res[SIGNAL], O_WRONLY | O_CREAT | O_TRUNC, 0644);
				fprintf(stdout, "downloading %s from (%s)...\n", res[SIGNAL], res[SENDER]);
				switch(status->opts->method) {
					case PIPES: 
						while((total += receive_pipe_file(self_read, nfd, status->opts, filesize)) < filesize);
						break;
					case SOCKETS:
						while((total += receive_sock_file(self_socket, nfd, status->opts, filesize)) < filesize);
					case QUEUE:
						while((total += receive_queue_file(self_queue, nfd, status->opts, filesize)) < filesize);
						break;
					default: break;
				}

				/* decompress file */
				if(status->opts->compress) inflate_file(res[SIGNAL], true);

				status->current_dir = get_dir_contents(status->dir);

				fprintf(stdout, "\nPress enter to continue...");
				while(getchar() != 10);

				/* here goes select function for choosing method of receiving */
				break;
			} case TOGGLE_ENCRYPTION:
				status->opts->encrypt = !status->opts->encrypt;
				send_message(self_write, MSG_ENCRYPT, true);
				break;
			case TOGGLE_COMPRESSION:
				status->opts->compress = !status->opts->compress;
				send_message(self_write, MSG_COMPRESS, true);
				break;
			case CLIENT_LS:
				dir_status = sprint_dir_status(status);
				info_screen(dir_status);
				break;
			case SET_MODE:
			{
				char mode[2];
				status->opts->method = method_menu();
				send_message(self_write, MSG_METHOD, true);
				snprintf(mode, 2, "%d", status->opts->method);
				send_message(self_write, mode, true);
				break;
			} 
			case CHANGE_CHUNKSIZE:
			{
				char strsize[MAX_BUFFER];
				do {
					fprintf(stdout, "\nEnter a new chunksize: ");
					status->opts->chunksize = atoi(fgets(strsize, MAX_BUFFER, stdin));
				}while(status->opts->chunksize < 0 || status->opts->chunksize > 4000000);
				int size = buffer_size("%d", status->opts->chunksize);
				char *str_chunksize = malloc(size);
				snprintf(str_chunksize, size, "%d", status->opts->chunksize);
				send_message(self_write, MSG_CHUNKSIZE, true);
				send_message(self_write, str_chunksize, true);
				break;
			} case TOGGLE_DEBUG:
				status->opts->debug = !status->opts->debug;
				//send message
				break;
			case EXIT: 
				send_message(self_write, MSG_EXIT, true);
				exit(0);
			default: break;
		}
	}
	return 0;
}
