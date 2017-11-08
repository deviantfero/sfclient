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
	char **response, *dir_status;
	int tmp_buffer_size = 0;

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
	send_message(sfs_path, MSG_ARRIVE, true);
	response = wait_message(sfs_path, DFT_TRIES);
	status->server_pid = atoi(response[SENDER]);
	status->server_dir = response[SIGNAL];

	while(opt != EXIT) {
		system("clear");
		opt = run_menu(status->opts);
		switch(opt) {
			case SERVER_LS:
				send_message(self_write, MSG_LS, true);
				response = wait_message(self_read, DFT_TRIES);
				info_screen(response[SIGNAL]);
				break;
			case SERVER_STATE:
				send_message(self_write, MSG_STATUS, true);
				response = wait_message(self_read, DFT_TRIES);
				info_screen(response[SIGNAL]);
				break;
			case UPLD_FILE:
				opt = choose_file(file_menu, status->current_dir->file_count);
				send_message(self_write, MSG_UPLD, true);
				send_message(self_write, status->current_dir->files[opt], true);
				upload_file(self_write, status->current_dir->files[opt], 0, NULL);
				fprintf(stdout, "Press enter to continue...");
				while(getchar() != 10);
				break;
			case DWNLD_FILE:
				//download file
				break;
			case TOGGLE_ENCRYPTION:
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
