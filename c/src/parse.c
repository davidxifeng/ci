#include <stdio.h>
#include <stdlib.h>

#include "ci.h"

extern char *data, *bd;

extern int *be, loc;
extern int line, *e, ival, *id, *sym;
extern enum Token tk;

/**
编译
*/
int parse() {
	int bt, ty;
	int i;
	// parse declarations
	line = 1;
	next();
	while (tk) {
		// 解析声明的类型
		bt = INT; // basetype
		if (tk == Int) {
			next();
		} else if (tk == Char) {
			next();
			bt = CHAR;
		} else if (tk == Enum) {
			next();
			if (tk != '{')
				next();
			if (tk == '{') {
				next();
				int enum_value = 0;
				while (tk != '}') {
					if (tk != Id) {
						printf("%d: bad enum identifier %d\n", line, tk);
						return -1;
					}
					next();
					if (tk == Assign) {
						next();
						if (tk != Num) {
							printf("%d: bad enum initializer\n", line);
							return -1;
						}
						enum_value = ival;
						next();
					}
					id[Class] = Num;
					id[Type] = INT;
					id[Val] = enum_value++;
					if (tk == ',')
						next();
				}
				next();
			}
		}

		// 解析函数 或变量声明
		while (tk != ';' && tk != '}') {
			ty = bt;
			while (tk == Mul) {
				next();
				ty = ty + PTR;
			}
			if (tk != Id) {
				printf("%d: bad global declaration\n", line);
				return -1;
			}
			if (id[Class]) {
				printf("%d: duplicate global definition\n", line);
				return -1;
			}
			next();
			id[Type] = ty;
			if (tk == '(') { // function
				id[Class] = Fun;
				id[Val] = (int)(e + 1 - be); //(int)(e + 1);
				next();			     // skip (

				i = 0; // param & local count

				// parse parameters
				while (tk != ')') {
					ty = INT;
					if (tk == Int) {
						next();
					} else if (tk == Char) {
						next();
						ty = CHAR;
					}
					while (tk == Mul) {
						next();
						ty = ty + PTR;
					}
					if (tk != Id) {
						printf("%d: bad parameter declaration\n", line);
						return -1;
					}
					if (id[Class] == Loc) {
						printf("%d: duplicate parameter definition\n",
						       line);
						return -1;
					}
					id[HClass] = id[Class];
					id[Class] = Loc;
					id[HType] = id[Type];
					id[Type] = ty;
					id[HVal] = id[Val];
					id[Val] = i++;
					next();
					if (tk == ',')
						next();
				}

				next(); // skip )

				// 错误检查 函数体开始的 {
				if (tk != '{') {
					printf("%d: bad function definition\n", line);
					return -1;
				}
				loc = ++i;
				next();

				// parse function body

				// 解析局部变量声明区
				while (tk == Int || tk == Char) {
					bt = (tk == Int) ? INT : CHAR;
					next();
					while (tk != ';') {
						ty = bt;
						while (tk == Mul) {
							next();
							ty = ty + PTR;
						}
						if (tk != Id) {
							printf("%d: bad local declaration\n", line);
							return -1;
						}
						if (id[Class] == Loc) {
							printf("%d: duplicate local definition\n",
							       line);
							return -1;
						}
						id[HClass] = id[Class];
						id[Class] = Loc;
						id[HType] = id[Type];
						id[Type] = ty;
						id[HVal] = id[Val];
						id[Val] = ++i;
						next();
						if (tk == ',')
							next();
					}
					next();
				}
				*++e = ENT;
				*++e = i - loc;
				// 解析函数语句, c4中语句区不能定义局部变量,做法类似
				// Pascal?, 或者早期C语言?
				while (tk != '}')
					stmt();
				if (*e != LEV) {
					*++e = LEV;
				}
				id = sym; // unwind symbol table locals
				while (id[Tk]) {
					if (id[Class] == Loc) {
						id[Class] = id[HClass];
						id[Type] = id[HType];
						id[Val] = id[HVal];
					}
					id = id + Idsz;
				}
			} else {
				// 给全局变量在 数据段分配空间.
				id[Class] = Glo;
				id[Val] = data - bd;
				data = data + 4;
				if (tk == ',')
					next();
			}
		}
		next();
	}
	return 0;
}

// vim: tabstop=2 shiftwidth=2 softtabstop=2
