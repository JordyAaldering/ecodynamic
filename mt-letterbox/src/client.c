#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#define SOCKET_PATH "/tmp/rust_unix_socket"

int main() {
    int sockfd;
    struct sockaddr_un addr;
    int number = 42;
    int received_number;

    // Create a Unix domain socket
    sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sockfd == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // Set up the socket address structure
    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);

    // Connect to the server
    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) == -1) {
        perror("connect");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    printf("Sending: %d\n", number);

    // Send the integer to the server
    if (write(sockfd, &number, sizeof(int)) == -1) {
        perror("write");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    // Read the incremented number from the server
    if (read(sockfd, &received_number, sizeof(int)) == -1) {
        perror("read");
        close(sockfd);
        exit(EXIT_FAILURE);
    }

    printf("Received: %d\n", received_number);

    // Close the socket
    close(sockfd);
    return 0;
}
