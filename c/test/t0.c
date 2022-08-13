#include <stdio.h>

int i = 1;
int j, k;

// 这样的初始化C不支持
//int l, m = 2, 3;

int l = 2, m = 3;
int n = 2, o;

int main() {
	printf("hello \a \b \r\n \v \tt\"\"' %d %d %d\n", i,2,3);
	return 0;
}
