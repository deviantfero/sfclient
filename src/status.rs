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
