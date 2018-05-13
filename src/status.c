#include "status.h"

void fprint_dir_status(FILE *file, struct client_status *s) {
	for(int i = 0; i < s->current_dir->file_count; i ++) {
		fprintf(file, "[%d]%30.30s\n", i + 1, s->current_dir->files[i]);
	}
}

char *sprint_dir_status(struct client_status *s) {
	size_t file_str_len = 0;
	char *file_str;
	char *str = malloc(1);

	for(int i = 0; i < s->current_dir->file_count; i ++) {
		/* get size of resulting string */
		file_str_len = buffer_size("[%d]%30.30s\n", i + 1, s->current_dir->files[i]);

		file_str = malloc(file_str_len);

		snprintf(file_str, file_str_len, "[%d]%30.30s\n", i + 1, s->current_dir->files[i]);

		str = realloc(str, strlen(str) + MAX_BUFFER);
		str = strncat(str, file_str, strlen(str) + file_str_len);

		free(file_str);
	}

	return str;
}


struct directory *get_dir_contents(const char *dir) {
	struct dirent *dir_contents;
	struct directory *current_dir = malloc(sizeof(struct directory));
	current_dir->files = malloc(sizeof(struct file_status*));

	if(current_dir == NULL) {
		return NULL;
	}

	DIR *fd = opendir(dir);

	for(int i = 0; (dir_contents = readdir(fd)) != NULL;){
		if(dir_contents->d_type == DT_REG) {
			current_dir->files = realloc(current_dir->files, sizeof(char*) * (i + 1));
			current_dir->files[i] = dir_contents->d_name;
			current_dir->file_count = i + 1;

			if(current_dir->files[i] == NULL) {
				printf("null at %d\n fetching files", i);
			}

			i++;
		}

	}

	return current_dir;
}
