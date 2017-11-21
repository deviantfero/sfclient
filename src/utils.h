#ifndef UTILS_H
#define UTILS_H

#define KEY "secretkey"

#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <zlib.h>

int buffer_size(const char* format, ...);
void encrypt(char *message, char *key, ssize_t chunksize);
char *chunk_deflate(char *str);
char *chunk_inflate(char *str);

#endif
