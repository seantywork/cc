#ifndef SERVER_ST_HEADER
#define SERVER_ST_HEADER

#include <stdio.h> 
#include <netdb.h> 
#include <netinet/in.h> 
#include <stdlib.h> 
#include <string.h> 
#include <sys/socket.h> 
#include <sys/types.h> 
#include <unistd.h> 
#include <time.h>
// ep headers
#include <fcntl.h>
#include <sys/epoll.h>
#include <errno.h>
#include <arpa/inet.h>
#include <pthread.h>


#endif // SERVER_ST_HEADER

#ifndef SERVER_EP_HEADER_DEF
#define SERVER_EP_HEADER_DEF

#define TRUE 1
#define FALSE 0
#define MAX_BUFF 1024 
#define MAX_CONN 80
#define PORT 8080 
#define SA struct sockaddr 
   

typedef struct async_pool {
    int fd;
    uint8_t *data;
    pthread_mutex_t lock;
    pthread_cond_t cond;
} async_pool;

extern struct sockaddr_in SERVADDR;
extern int SERVLEN;

extern int SOCKFD;
extern int EPLFD;
extern int MAX_SD;

extern int OPT;

extern struct epoll_event EVENT;
extern struct epoll_event *CLIENT_SOCKET;
extern pthread_t *WORK_PID;
extern pthread_t *WRITE_PID;
extern int *APOOL_ID;
extern async_pool *APOOL;
extern async_pool *AWPOOL;


int make_socket_non_blocking (int);

void handle_conn();

void handle_client(int);

void* worker(void* varg);

void* writer(void* varg);


void to_worker(int id, int fd, uint8_t *data);

void to_writer(int id, int fd, uint8_t *data);

#endif