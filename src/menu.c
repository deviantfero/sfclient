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
	printf("\t7. See client content\n");
	printf("\t8. Set transfer method [%s]\n", get_method_name(copt->method));
	puts("\t9. Exit");
	printf(PROMPT);
}

char* get_method_name(enum method m) {
	switch(m) {
		case PIPES:   return "pipes";
		case QUEUE:   return "queue";
		case SOCKETS: return "sockets";
		default:      return "unknown";
	}
}

int method_menu() {
	enum method m;
	char* tmp = malloc(BUFFER_MAX);
	if(tmp == NULL) return PIPES;

	do {
		for(int i = 0; i <= SOCKETS; i++)
			fprintf(stdout, "[%d] %s\n", i, get_method_name(i));
		fprintf(stdout, "\nEnter a method number: ");
		m = atoi(fgets(tmp, BUFFER_MAX, stdin));
	} while(m < PIPES || m > SOCKETS);

	return m;
}

int run_menu(struct options *copt) {
	enum menu_opt opt;
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

int choose_file(const char *dir_status, int file_count) {
	char *tmp = malloc(BUFFER_MAX);
	if(tmp == NULL) exit(2);
	int opt;
	do {
		fprintf(stdout, "%s\nInput a file number: ", dir_status);
		fgets(tmp, BUFFER_MAX, stdin);
		opt = atoi(tmp);
	} while(opt < 1 || opt > file_count);

	return opt - 1;
}

void info_screen(char *info) {
	system("clear");
	fprintf(stdout, "%s\nPress enter", info);
	getchar();
	system("clear");
}

struct options *get_default_opts() {
	struct options *default_opts = malloc(sizeof(default_opts));
	default_opts->encrypt = false;
	default_opts->compress = false;
	default_opts->method = 0;
	default_opts->chunk_size = 0;

	return default_opts;
}
