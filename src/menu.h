#ifndef MENU_H

#define BUFFER_MAX 100
#define PROMPT "Enter a number: "
#include <stdbool.h>

enum menu_opt {
	NOT_VALID, 
	SERVER_LS, 
	SERVER_STATE, 
	UPLD_FILE, 
	DWNLD_FILE, 
	TOGGLE_ENCRYPTION,
	TOGGLE_COMPRESSION,
	EXIT
};

struct options {
	bool encrypt;
	bool compress;
	int method;
};

void print_menu(struct options *copt);
int run_menu(struct options *copt);
struct options *get_default_opts();

#define MENU_H
#endif /* ifndef MENU_H */
