#include <stdio.h>
int fail_bz(int i) { if (i > 2) { printf(">2\n"); } return 0; }
int no_return_one_lev(int i) { if (i > 2) { printf(">2\n"); } }
int main(int argc, char ** argv) {
    int x;
    fail_bz(argc);
    if (argc % 2) { printf("true\n"); } else { printf("false\n"); } x = argc % 2 ? 3 : 2; printf("x is %d\n", x);
    while ( --x > 0) { printf("px\n"); } return 0; }

