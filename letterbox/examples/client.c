#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#define SOCKET_PATH "/tmp/mtdynamic_letterbox"

struct Send {
    int pid;
    int fid;
    float value;
};

void foo(void) { }

int main() {
    // Create a Unix domain socket
    int sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sockfd == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // Set up the socket address structure
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);

    // Connect to the server
    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        perror("connect");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    struct Send send;
    send.pid = (int)getpid();
    send.fid = (int)((uintptr_t)foo);
    send.value = 1.234;

    printf("Sending: (%d, %d) -> %f\n", send.pid, send.fid, send.value);

    // Send the integer to the server
    if (write(sockfd, &send, sizeof(send)) == -1) {
        perror("write");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    // Read the incremented number from the server
    int received;
    if (read(sockfd, &received, sizeof(int)) == -1) {
        perror("read");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    printf("Received: %d\n", received);

    // Close the socket
    close(sockfd);
    return 0;
}
