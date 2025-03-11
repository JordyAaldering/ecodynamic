#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#include "../../letterbox.h"

void foo(void) { }

int main() {
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
    strncpy(addr.sun_path, MTD_LETTERBOX_PATH, sizeof(addr.sun_path) - 1);

    // Connect to server
    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        perror("connect");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    while (1) {
        // Write to stream
        struct Incoming send;
        send.uid = (int)((uintptr_t)foo);
        send.val = 1.234;
        printf("Send: (%d, %f)\n", send.uid, send.val);

        if (write(sockfd, &send, sizeof(send)) == -1) {
            perror("write");
            close(sockfd);
            exit(EXIT_FAILURE);
        }

        // Read from stream
        struct Outgoing msg;

        if (read(sockfd, &msg, sizeof(msg)) == -1) {
            perror("read");
            close(sockfd);
            exit(EXIT_FAILURE);
        }

        sleep(1);

        printf("Recv: %d\n", msg.threads);
    }

    // Close the socket
    close(sockfd);
    return 0;
}
