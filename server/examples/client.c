#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#include "../../target/debug/letterbox.h"

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
    strncpy(addr.sun_path, MTD_LETTERBOX_PATH, sizeof(addr.sun_path) - 1);

    // Connect to server
    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        close(sockfd);
        perror("connect");
        exit(EXIT_FAILURE);
    }

    printf("Successfully opened letterbox\n");
    return sockfd;
}

struct Demand read_letterbox(int sockfd, uintptr_t fptr) {
    if (sockfd < 0) {
        // Initialize the socket

    }

    // Signal to the controller which region we are about to start
    struct Request request;
    request.region_uid = (int32_t)fptr;
    request.problem_size = 0;
    if (write(sockfd, &request, sizeof(struct Request)) == -1) {
        close(sockfd);
        perror("write");
        exit(EXIT_FAILURE);
    }

    // Read the demand of this region from the controller
    struct Demand demand;
    if (read(sockfd, &demand, sizeof(struct Demand)) == -1) {
        close(sockfd);
        perror("read");
        exit(EXIT_FAILURE);
    }

    return demand;
}

void update_letterbox(int sockfd, uintptr_t fptr) {
    // Send runtime metrics to controller
    struct Sample sample;
    sample.region_uid = (int32_t)fptr;
    sample.runtime = 1.234;
    sample.usertime = 2.345;
    sample.energy = 3.456;
    if (write (sockfd, &sample, sizeof(struct Sample)) == -1) {
        close(sockfd);
        perror("write");
        exit(EXIT_FAILURE);
    }
}

/* Some function we are controlling. */
void foo(void) {
    sleep(1);
}

int main() {
    int sockfd = open_letterbox();

    while (1) {
        struct Demand demand = read_letterbox(sockfd, (uintptr_t)foo);
        printf("Received demand: %f\n", demand.threads_pct);

        foo();

        printf("Sending runtime metrics\n");
        update_letterbox(sockfd, (uintptr_t)foo);
    }
}
