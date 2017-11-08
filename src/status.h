#ifndef STATUS_H
#define STATUS_H

#define DEFAULT_DIR "./"

#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <dirent.h>
#include <string.h>
#include "utils.h"
#include "comms.h"



struct directory {
	char **files;
	int file_count;
};

struct client_status {
	const char *dir;
	const char *server_dir;
	struct directory *current_dir;
	struct options *opts;
	int pid;
	int server_pid;
};


/* receives a client_status struct and prints it's directories
 * and stats on how many times they've been downloaded into a
 * file specified by the [file] argument */
void fprint_dir_status(FILE* file, struct client_status *status);

/* receives a client_status struct and prints it's directories
 * and stats on how many times they've been downloaded into a
 * string specified by the [str] argument */
char *sprint_dir_status(struct client_status *status);

/* receives a client_status struct and prints it's contents
 * into FILE* specified by file param */
void fprint_status(FILE *file, struct client_status *status);

/* receives a client_status struct and prints it's current status
 * string specified by the [str] argument */
char *sprint_status(struct client_status *status);

/* returns an array containing the names of
 * the directory's contents receives the 
 * directory path as argument */
struct directory *get_dir_contents(const char* dir);

#endif /* STATUS_H defined */
