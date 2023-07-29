#include "stdlib.h"
#include <stdint.h>
#include <stdio.h>
#include "unistd.h"

void print(uint64_t x) {
    char buf[32];
    size_t buf_sz = 1;
    buf[sizeof(buf) - buf_sz] = '\n';
    do {
        buf[sizeof(buf) - buf_sz - 1] = x % 10 + '0';
        buf_sz++;
        x /= 10;
    } while (x > 0);
    write(1, &buf[sizeof(buf) - buf_sz ], buf_sz);
}

int main() {
    print(34);
    return 0;
}
