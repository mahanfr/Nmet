#include <bits/sockaddr.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

struct sockaddr_in server_addr, cli;

int main(void) {
    // printf("AF_INET = %d\n",AF_INET);
    // printf("SOCK_STREAM = %d\n",SOCK_STREAM);
    // printf("INADDR_ANY= %d\n",INADDR_ANY);
    // printf("%X\n", htons(6969));
    // printf("%X\n", inet_addr("127.0.0.1"));
    // printf("size of sockaddr_in: %ld\n",sizeof(server_addr));
    // printf("size of sa_family_t: %ld\n",sizeof(server_addr.sin_family));
    // printf("size of in_port_t: %ld\n",sizeof(server_addr.sin_port));
    // printf("size of struct in_addr: %ld\n",sizeof(server_addr.sin_addr));
    // printf("size of sin_zero: %ld\n",sizeof(server_addr.sin_zero));
    // printf("size of s_addr inside in_addr: %ld\n",sizeof(server_addr.sin_addr.s_addr));
    // printf("%d\n",htons(8000));
    struct sockaddr_in serveraddr;
    int socket_fd = socket(2, 1, 0);
    if (socket_fd < 0) {
        exit(-1);
    }
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    server_addr.sin_port = htons(8000);
    if(bind(socket_fd, (struct sockaddr*) 0, 0) < 0) {
        exit(45);
    }
    if ((listen(socket_fd, 5)) < 0) { 
        printf("Listen failed...\n"); 
        exit(0); 
    }
    socklen_t len = sizeof(cli);
    int connfd = accept(socket_fd, (struct sockaddr*)&cli, &len); 
    if (connfd < 0) { 
        printf("server accept failed...\n"); 
        exit(0); 
    }
    printf("Success!\n");
    return 0;
}
