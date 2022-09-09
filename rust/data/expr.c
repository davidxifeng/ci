#include <stdio.h>

int a = 2;

int main()
{
	int i;
	int b[a == 3 ? 2 : 3];
	int b2[a = 1];

	// 逗号运算符, ()表达式,从左至右求值,丢弃结果,只保留最后一个的作为整个表达式的值
	i = (i = 0, i++, --i, ++i);
	printf("i is: %d\n", i); // 1

	int j = 1, k = 2, l;

	// >	assignment-expression:
	// >		conditional-expression
	// >		unary-expression assignment-operator assignment-expression

	// 语法上,没有赋值操作符的 赋值表达式
	i ? j : k;

	l = i != 1 ? j : k;

	printf("l is: %d\n", l); // 1

	// gcc编译: 报错
	// g++编译: 顺利通过
	// l = i ? j : k = 3; // error
	// l = i ? j : k = 3;
	//               ^
	//               |
	// error: lvalue required as left operand of assignment

	l = i ? j : (k = 3);	 // ok
	printf("l is: %d\n", l); // 1

	return 0;
}
