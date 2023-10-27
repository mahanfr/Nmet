#include <bits/sockaddr.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <netinet/in.h>

struct sockaddr_in server_addr;

int main(void) {
    printf("AF_INET = %d\n",AF_INET);
    printf("SOCK_STREAM = %d\n",SOCK_STREAM);
    printf("INADDR_ANY= %d\n",INADDR_ANY);
    int socket_fd = socket(AF_INET, SOCK_STREAM, 0);

    printf("size of sockaddr_in: %ld\n",sizeof(server_addr));
    printf("size of sa_family_t: %ld\n",sizeof(server_addr.sin_family));
    printf("size of in_port_t: %ld\n",sizeof(server_addr.sin_port));
    printf("size of struct in_addr: %ld\n",sizeof(server_addr.sin_addr));
    printf("size of sin_zero: %ld\n",sizeof(server_addr.sin_zero));
    printf("size of s_addr inside in_addr: %ld\n",sizeof(server_addr.sin_addr.s_addr));
    printf("%d\n",htons(8000));
    if (socket_fd < 0) {
        exit(-1);
    }
    return 0;
}
