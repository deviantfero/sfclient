#ifndef TRANSFER_H

#define TRANSFER_H

#define TRANSFER_CHAR '#'

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <stddef.h>
#include <fcntl.h>
#include <errno.h>
#include "comms.h"
#include "utils.h"
#include "menu.h"


int upload_file(const char *pipe_name, char *src, int chunksize, enum method *m);
void fprogress_bar(FILE *file, off_t total_size, size_t transfered);
int send_pipe_file(const char *pipe_name, int src_fd, int chunksize, size_t file_size);
int send_sock_file(const char *sock_name, int src_fd, int chunksize, size_t file_size);
int make_named_sock(const char* sock_name, bool recv);
int receive_pipe_file(const char *pipe_name, int piped, int chunksize, size_t filesize);
int receive_sock_file(const char *sock_name, int dst_fd, int chunksize, size_t filesize);

#endif /* ifndef TRANSFER_H */
