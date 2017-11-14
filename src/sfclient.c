#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <limits.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/ioctl.h>

#include "comms.h"
#include "menu.h"
#include "status.h"
#include "transfer.h"


struct client_status *status;

int main(void) {
	const char* sfs_path = "/tmp/sfs";
	/* two pipes per client for an individual thread
	 * in the server, one for reading, one for writing. */
	char *self_read = malloc(MAX_BUFFER); 
	char *self_write = malloc(MAX_BUFFER);
	char **res, *dir_status;
	char *server_files;
	char* tmp_buffer; //multiple uses 
	int server_file_amount;

	status = malloc(sizeof(struct client_status*));

	snprintf(self_read, MAX_BUFFER, "/tmp/sfc%dr", getpid());
	snprintf(self_write, MAX_BUFFER, "/tmp/sfc%dw", getpid());

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
				upload_file(self_write, status->current_dir->files[opt], 0, NULL);
				fprintf(stdout, "Press enter to continue...");
				while(getchar() != 10);
				break;
			case DWNLD_FILE: 
			{ 
				int total = 0;
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
				
				/* receive filename, filesize and chunksize */
				res = wait_message(self_read, DFT_TRIES);
				int chunksize = atoi(res[SIGNAL]);
				res = wait_message(self_read, DFT_TRIES);
				int filesize = atoi(res[SIGNAL]);
				res = wait_message(self_read, DFT_TRIES);

				int nfd = open(res[SIGNAL], O_WRONLY | O_CREAT | O_TRUNC, 0644);
				fprintf(stdout, "downloading %s from (%s)...\n", res[SIGNAL], res[SENDER]);
				while((total += receive_pipe_file(self_read, nfd, chunksize, filesize)) < filesize);
				fprintf(stdout, "Press enter to continue...");
				while(getchar() != 10);

				/* here goes select function for choosing method of receiving */
				break;
			} case TOGGLE_ENCRYPTION:
				status->opts->encrypt = !status->opts->encrypt;
				break;
			case TOGGLE_COMPRESSION:
				status->opts->compress = !status->opts->compress;
				break;
			case CLIENT_LS:
				dir_status = sprint_dir_status(status);
				info_screen(dir_status);
				break;
			case NOT_VALID: break;
			case EXIT: 
				send_message(self_write, MSG_EXIT, true);
				exit(0);
			default: break;
		}
	}
	return 0;
}
