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
        struct Sample sample;
        sample.max = 16;
        sample.uid = (int)((uintptr_t)foo);
        sample.val = 1.234;
        printf("Send: (%d, %f)\n", sample.uid, sample.val);

        if (write(sockfd, &sample, sizeof(sample)) == -1) {
            perror("write");
            close(sockfd);
            exit(EXIT_FAILURE);
        }

        // Read from stream
        struct Demand demand;

        if (read(sockfd, &demand, sizeof(demand)) == -1) {
            perror("read");
            close(sockfd);
            exit(EXIT_FAILURE);
        }

        sleep(1);

        printf("Recv: %d\n", demand.threads);
    }

    // Close the socket
    close(sockfd);
    return 0;
}
