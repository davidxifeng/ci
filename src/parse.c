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
		bt = INT; // basetype
		if (tk == Int) {
			next();
		} else if (tk == Char) {
			next();
			bt = CHAR;
		} else if (tk == Enum) {
			next();
			if (tk != '{') next();
			if (tk == '{') {
				next();
				i = 0;
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
						i = ival;
						next();
					}
					id[Class] = Num; id[Type] = INT; id[Val] = i++;
					if (tk == ',') next();
				}
				next();
			}
		}

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
				next(); i = 0;
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
						printf("%d: duplicate parameter definition\n", line);
						return -1;
					}
					id[HClass] = id[Class];
					id[Class]  = Loc;
					id[HType]  = id[Type];
					id[Type]   = ty;
					id[HVal]   = id[Val];
					id[Val]    = i++;
					next();
					if (tk == ',') next();
				}
				next();
				if (tk != '{') {
					printf("%d: bad function definition\n", line);
					return -1;
				}
				loc = ++i;
				next();
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
							printf("%d: duplicate local definition\n", line);
							return -1;
						}
						id[HClass] = id[Class];
						id[Class]  = Loc;
						id[HType]  = id[Type];
						id[Type]   = ty;
						id[HVal]   = id[Val];
						id[Val]    = ++i;
						next();
						if (tk == ',') next();
					}
					next();
				}
				*++e = ENT; *++e = i - loc;
				while (tk != '}') stmt();
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
				id[Class] = Glo;
				id[Val] = data - bd;
				data = data + 4;
				if (tk == ',') next();
			}
		}
		next();
	}
	return 0;
}


// vim: tabstop=2 shiftwidth=2 softtabstop=2
