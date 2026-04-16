#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#define LETTERBOX_PATH "/tmp/mtd_letterbox"

static int parse_num_threads(const char *json, uint16_t *value) {
    const char *key = "\"num_threads\":";
    const char *pos = strstr(json, key);
    char *end = NULL;

    if (pos == NULL) {
        return 0;
    }

    pos += strlen(key);
    // Skip potential whitespace in between the key and the value
    while (*pos != '\0' && isspace((unsigned char)*pos)) {
        pos++;
    }

    *value = strtof(pos, &end);
    return end != pos;
}

int open_letterbox(void) {
    // Create Unix domain socket
    int sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sockfd == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // Set up socket address structure
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, LETTERBOX_PATH, sizeof(addr.sun_path) - 1);

    // Connect to server
    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        close(sockfd);
        perror("connect");
        exit(EXIT_FAILURE);
    }

    printf("Successfully opened letterbox\n");
    return sockfd;
}

void read_letterbox(FILE *sock_file, int32_t region_uid) {
    // Signal to the controller which region we are about to start
    fprintf(sock_file, "{\"region_uid\":%d}\n", region_uid);
    fflush(sock_file);

    // Read the demand of this region from the controller
    char buf[256];
    if (fgets(buf, sizeof(buf), sock_file) == NULL) {
        perror("fgets");
        exit(EXIT_FAILURE);
    }

    uint16_t num_threads;
    if (parse_num_threads(buf, &num_threads)) {
        printf("num_threads: %u\n", num_threads);
    } else {
        printf("num_threads not found in demand\n");
    }

    printf("Received demand: %s", buf);
}

void update_letterbox(FILE *sock_file, int32_t region_uid) {
    // Send runtime metrics to controller
    fprintf(sock_file, "{\"region_uid\":%d,\"runtime\":1.234,\"usertime\":2.345,\"energy\":3.456}\n", region_uid);
    fflush(sock_file);
}

/* Some function we are controlling. */
void foo(void) {
    sleep(1);
}

int main() {
    int sockfd = open_letterbox();
    FILE *sock_file = fdopen(sockfd, "r+");
    if (sock_file == NULL) {
        perror("fdopen");
        exit(EXIT_FAILURE);
    }

    // Broadcast capabilities on connect
    fprintf(sock_file, "{\"max_threads\":4}\n");
    fflush(sock_file);

    while (1) {
        read_letterbox(sock_file, 42);

        foo();

        printf("Sending runtime metrics\n");
        update_letterbox(sock_file, 42);
    }
}
