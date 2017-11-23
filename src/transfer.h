#ifndef TRANSFER_H

#define TRANSFER_H

#define TRANSFER_CHAR '#'
#define PRIORITY 5u

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <mqueue.h>
#include <sys/un.h>
#include <stddef.h>
#include <fcntl.h>
#include <errno.h>
#include "utils.h"
#include "comms.h"
#include "menu.h"


ssize_t upload_file(const char *pipe_name, char *src, struct options *opt);
void fprogress_bar(FILE *file, off_t total_size, size_t transfered);
ssize_t send_pipe_file(const char *pipe_name, int src_fd, struct options *opt, size_t file_size);
ssize_t send_sock_file(const char *sock_name, int src_fd, struct options *opt, size_t file_size);
ssize_t send_queue_file(const char *queue, int src_fd, struct options *opt, size_t file_size);
int make_named_sock(const char* sock_name, bool recv);
ssize_t receive_pipe_file(const char *pipe_name, int piped, struct options *opt, size_t filesize);
ssize_t receive_sock_file(const char *sock_name, int dst_fd, struct options *opt, size_t filesize);
ssize_t receive_queue_file(const char *queue, int src_fd, struct options *opt, size_t file_size);

#endif /* ifndef TRANSFER_H */
