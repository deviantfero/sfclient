#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <limits.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include "comms.h"
#include "menu.h"


int main(void) {
	printf(":: Simple File Client\n");

	const char* fifo_path = "/tmp/fifo";

	struct options *active_opts = get_default_opts();
	enum menu_opt opt;

	while(opt != EXIT) {
		opt = run_menu(active_opts);
		system("clear");
		switch(opt) {
			case SERVER_LS:
				send_message(fifo_path, MSG_LS, true);
				// wait message in process fifo file
				/* wait_message("/tmp/fifo2"); */
				break;
			case SERVER_STATE:
				//call server state here
				break;
			case UPLD_FILE:
				//upload file
				break;
			case DWNLD_FILE:
				//download file
				break;
			case TOGGLE_ENCRYPTION:
				active_opts->encrypt = !active_opts->encrypt;
				break;
			case TOGGLE_COMPRESSION:
				active_opts->compress = !active_opts->compress;
				break;
			case NOT_VALID: break;
			case EXIT: 
				send_message(fifo_path, MSG_EXIT, true);
				exit(0);
		}

	}
	return 0;
}
