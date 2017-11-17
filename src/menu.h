#ifndef MENU_H

#define BUFFER_MAX 100
#define PROMPT "Enter a number: "

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdbool.h>
#include "status.h"

enum menu_opt {
	NOT_VALID, 
	SERVER_LS, 
	SERVER_STATE, 
	UPLD_FILE, 
	DWNLD_FILE, 
	TOGGLE_ENCRYPTION,
	TOGGLE_COMPRESSION,
	CLIENT_LS,
	SET_MODE,
	EXIT
};

struct options {
	bool encrypt;
	bool compress;
	int chunk_size;
	int method;
};

enum method {
	PIPES,
	QUEUE,
	SOCKETS
};

char* get_method_name(enum method m );
int choose_file(const char *dir_status, int file_count);
int method_menu();
int run_menu(struct options *copt);
struct options *get_default_opts();
void info_screen(char *info);
void print_menu(struct options *copt);

#define MENU_H
#endif /* ifndef MENU_H */
