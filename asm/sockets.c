#include <asm-generic/errno-base.h>
#include <asm-generic/errno.h>
#include <bits/sockaddr.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

int main(void) {
    // printf("AF_INET = %d\n",AF_INET);
    // printf("SOCK_STREAM = %d\n",SOCK_STREAM);
    // printf("INADDR_ANY= %d\n",INADDR_ANY);
    // printf("%X\n", htons(8000));
    // printf("%X\n", inet_addr("127.0.0.1"));
    // printf("size of sockaddr_in: %ld\n",sizeof(server_addr));
    // printf("size of sa_family_t: %ld\n",sizeof(server_addr.sin_family));
    // printf("size of in_port_t: %ld\n",sizeof(server_addr.sin_port));
    // printf("size of struct in_addr: %ld\n",sizeof(server_addr.sin_addr));
    // printf("size of sin_zero: %ld\n",sizeof(server_addr.sin_zero));
    // printf("size of s_addr inside in_addr: %ld\n",sizeof(server_addr.sin_addr.s_addr));
    // printf("%d\n",htons(8000));

    // printf("EAGAIN      %d\n", EAGAIN);
    // printf("EBADF       %d\n", EBADF);
    // printf("ECONNABORTED %d\n", ECONNABORTED);
    // printf("EFAULT      %d\n", EFAULT);
    // printf("EINTR       %d\n", EINTR);
    // printf("EINVAL      %d\n", EINVAL);
    // printf("EMFILE      %d\n", EMFILE);
    // printf("ENFILE      %d\n", ENFILE);
    // printf("ENOBUFS     %d\n", ENOBUFS);
    // printf("ENOTSOCK    %d\n", ENOTSOCK);
    // printf("EOPNOTSUPP  %d\n", EOPNOTSUPP);
    // printf("EPERM       %d\n", EPERM);
    // printf("EPROTO      %d\n", EPROTO);
    // printf("IPPROTO_TCP %d\n", IPPROTO_TCP);
    struct sockaddr_in serveraddr;
    int socket_fd = socket(2, 1, 0);
    if (socket_fd < 0) {
        exit(-1);
    }
    // **
    char addr[16];
    addr[0] = 2;
    addr[1] = 0;
    addr[2] = 0x1b;
    addr[3] = 0x39;
    addr[4] = 0;
    addr[5] = 0;
    addr[6] = 0;
    addr[7] = 0;
    if(bind(socket_fd, (struct sockaddr*) &addr, 16) < 0) {
        perror("Bind Failed");
        exit(45);
    }
    if ((listen(socket_fd, 5)) < 0) { 
        printf("Listen failed...\n"); 
        exit(0); 
    }
    char cli[16];
    int a[1] = {0}; 
    a[0] = 16;
    printf("%d\n",a[0]);
    int connfd = accept(socket_fd, (struct sockaddr*)0, (socklen_t*)0); 
    if (connfd < 0) { 
        perror("Accept");
        exit(0); 
    }
    printf("Success!\n");
    return 0;
}
