#ifndef TRANSFER_H

#define TRANSFER_H

#define TRANSFER_CHAR '#'

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <fcntl.h>
#include "comms.h"
#include "utils.h"

enum method {
	PIPES,
	QUEUE,
	SOCKETS
};

void upload_file(const char *pipe_name, const char *src, size_t max_rate, enum method *m);
void fprogress_bar(FILE *file, off_t total_size, size_t transfered);

#endif /* ifndef TRANSFER_H */
