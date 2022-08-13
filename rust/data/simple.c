#include <stdio.h>

int fn(int i) {
	return i * 2;
}

int i = 1;

// 居然也是合法的声明.
int;

int main() {
	printf("i is %d\n", i);
	return 0;
}