#include "utils.h"

/* https://stackoverflow.com/questions/3919995/determining-sprintf-buffer-size-whats-the-standard 
 * original by: Regis Portales */

int buffer_size(const char* format, ...) {
    va_list args;
    va_start(args, format);
    int result = vsnprintf(NULL, 0, format, args);
    va_end(args);
    return result + 1; // safe byte for \0
}

/* original by: Tim
 * https://stackoverflow.com/questions/8072868/simple-xor-algorithm
 */

void encrypt(char *chunk, char * key, ssize_t chunksize) {
    ssize_t keylen = buffer_size("%s", key);
    for(ssize_t i = 0; i < chunksize; i++) {
        chunk[i] = chunk[i] ^ key[i % keylen];
    }
}

char* deflate_file(char *src) {
	int src_fd = open(src, O_RDONLY);
	ssize_t filesize = lseek(src_fd, 0, SEEK_END);
	lseek(src_fd, (off_t) 0, SEEK_SET);
    ssize_t gznamesize = buffer_size("%s.gz", src);
    char *gzsrc  = malloc(gznamesize);
    snprintf(gzsrc, gznamesize, "%s.gz", src);

    char *file = malloc(filesize + 1);
    struct gzFile_s *fi = (struct gzFile_s *)gzopen(gzsrc, "wb");
    read(src_fd, file, filesize);

    gzwrite(fi, file, filesize);
    gzclose(fi);

    free(file);
    return gzsrc;
}

ssize_t inflate_file(char *src, bool del) {
    int usrcsize = buffer_size("%s", src) - 4;
    char buffer[MAX_BUFFER];

    char *usrc = malloc(usrcsize);
    memset(usrc, 0, usrcsize);

    strncpy(usrc, src, usrcsize);
    usrc[usrcsize] = '\0';

    struct gzFile_s *fi = (struct gzFile_s *)gzopen(src, "rb");
    gzrewind(fi);


    int fd  = open(usrc, O_WRONLY | O_CREAT | O_TRUNC, 0666);
    int len = 0; 
    ssize_t total = 0;
    while(!gzeof(fi)) {
        len = gzread(fi, buffer, sizeof(buffer));
        total += len;
        write(fd, buffer, len);
    }

    gzclose(fi);
    close(fd);
    if(del) unlink(src);
    return total;
}
