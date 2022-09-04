#include <stdio.h>

int fn(int i) {
	printf("i is %d\n", i);
	return - - - i * 2;
}

int i = 1;

// 居然也是合法的声明.
int;

int main() {
	printf("i is %d\n", i);
	int k = 0;
	fn(i == 1 ? k+=1, k+=2 : 5) ;
	return 0;
}
