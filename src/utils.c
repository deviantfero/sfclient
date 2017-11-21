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

void encrypt(char *message, char * key, ssize_t chunksize) {
    ssize_t keylen = buffer_size("%s", key);
    for(ssize_t i = 0; i < chunksize; i++) {
        message[i] = message[i] ^ key[i % keylen];
    }
}

char *chunk_deflate(char *str) {
    size_t strsize = buffer_size("%s", str) + 2;
    char *deflated = malloc(strsize);
    memset(deflated, 0, strsize);
    z_stream defstream;
    memset(&defstream, 0, sizeof(defstream));
    defstream.zalloc = Z_NULL;
    defstream.zfree  = Z_NULL;
    defstream.opaque = Z_NULL;
    defstream.avail_in = (uInt)strsize;
    defstream.next_in  = (Bytef *)str;
    defstream.avail_out = (uInt)strsize;
    defstream.next_out = (Bytef *)deflated;

    deflateInit(&defstream, Z_DEFAULT_COMPRESSION);
    deflate(&defstream, Z_FINISH);
    deflateEnd(&defstream);

    return deflated;
}

char *chunk_inflate(char *str) {
    size_t strsize = buffer_size("%s", str) + 2;
    char *inflated = malloc(strsize);
    int ret;
    memset(inflated, 0, strsize);

    z_stream defstream;
    memset(&defstream, 0, sizeof(defstream));
    defstream.zalloc = Z_NULL;
    defstream.zfree  = Z_NULL;
    defstream.opaque = Z_NULL;
    defstream.avail_in = (uInt)strsize;
    defstream.next_in  = (Bytef *)str;

    if(inflateInit(&defstream) != Z_OK) return NULL;
    do {
        defstream.avail_out = (uInt)strsize;
        defstream.next_out = (Bytef *)inflated;
        ret = inflate(&defstream, Z_NO_FLUSH);
    }while(ret == Z_OK);

    inflateEnd(&defstream);
    return inflated;
}
