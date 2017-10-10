#include "menu.h"


void print_menu(struct options *copt) {
	char *encryption_entry = copt->encrypt ? "[on]" : "[off]";
	char *compression_entry = copt->compress ? "[on]" : "[off]";
	puts("\t1. See server content");
	puts("\t2. See server status");
	puts("\t3. Upload a file");
	puts("\t4. Download a file");
	printf("\t5. Encrypt transfer %s\n", encryption_entry);
	printf("\t6. Compress transfer %s\n", compression_entry);
	puts("\t7. Exit");
	printf(PROMPT);
}

int run_menu(struct options *copt) {
	enum menu_opt opt = NOT_VALID;
	char *tmp = malloc(BUFFER_MAX);
	if(tmp == NULL) exit(2);

	do {
		print_menu(copt);
		fgets(tmp, BUFFER_MAX, stdin);
		opt = atoi(tmp);
		if(opt == NOT_VALID) puts("Enter a valid option...");
	} while(opt == NOT_VALID);

	return opt;
}

struct options *get_default_opts() {
	struct options *default_opts = malloc(sizeof(default_opts));
	default_opts->encrypt = false;
	default_opts->compress = false;
	default_opts->method = 0;

	return default_opts;
}
