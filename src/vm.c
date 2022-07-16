
#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include "ci.h"

#define ci_dispatch(o)  switch(o)
#define ci_case(c,b)    case c: {b} break;
#define ci_default(b)   default: {b};

const char *op_codes =
	"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LGB ,"
	"LEV ,"
	"LI  ,LC  ,SI  ,SC  ,PSH ,"

	"OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,"
	"SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,"

	"OPEN,READ,CLOS,PRTF,MALC,MSET,MCMP,EXIT,";

// 代码段基地址
extern int * be;

static void
dump_instruction(int *pc, int i, int cycle) {
	printf(COLOR_GREEN "%d> %d: %.4s", cycle, (int)(pc - 1 - be), &op_codes[i * 5]);

	// 本组指令有一个操作数
	if (i <= LGB) {
		printf(" %d\n" COLOR_RESET, *pc);
	} else {
		printf("\n" COLOR_RESET);
	}
}

extern int * sym; // 符号表
extern char * bd; // 数据段基地址

int run_c(int argc, char **argv, int debug, int main_addr) {
	int *pc, *sp; // 栈地址
	int *bp = NULL, a = 0, cycle = 0; // vm registers

	// 解析运行时,从符号表中查找main函数的地址
	if (main_addr == -1) {
		int *id = sym;
		while (id[Tk]) {
			if (!memcmp((const void *)id[Name], "main", 4)) {
				break;
			}
			id = id + Idsz;
		}

		if (!(pc = be + id[Val])) {
			printf("main() not defined\n");
			return -1;
		}
	} else {
		 pc = be + main_addr;
	}

	int stack_size = 128 * 1024;
	if (!(sp = malloc(stack_size * sizeof(int)))) {
		printf("could not malloc(%d) stack area\n", stack_size);
		return -1;
	}

	int i, *t; // temps
	// setup stack
	sp    = sp + stack_size;
	*--sp = EXIT; // call exit if main returns
	*--sp = PSH;

	t     = sp;
	*--sp = argc;
	*--sp = (int)argv;
	*--sp = (int)t;

	while (1) {
		i = *pc++; ++cycle;
		if (debug) dump_instruction(pc, i, cycle);

		ci_dispatch(i) {
			ci_case(LEA, a = (int)(bp + *pc++);)                         // load local address
			ci_case(IMM, a = *pc++;)                                     // load immediate
			ci_case(JMP, pc = pc + *pc;)                                 // jump
			ci_case(JSR, *--sp = (int)(pc + 1); pc = be + *pc;)          // jump to subroutine
			ci_case(BZ,  pc = a ? pc + 1 : pc + *pc;)                    // branch if zero
			ci_case(BNZ, pc = a ? pc + *pc : pc + 1;)                    // branch if not zero
			ci_case(ENT, *--sp = (int)bp; bp = sp; sp = sp - *pc++;)     // enter subroutine
			ci_case(ADJ, sp = sp + *pc++;)                               // stack adjust

			// global： 指针或者是一个int值，存入到栈上
			ci_case(LGB, a = (int)(bd + *pc++);)                         // load global address
			ci_case(LEV, sp = bp; bp = (int *)*sp++; pc = (int *)*sp++;) // leave subroutine

			ci_case(LI,  a = *(int *)a;)          // load int
			ci_case(LC,  a = *(char *)a;)         // load char
			ci_case(SI,  *(int *)*sp++ = a;)      // store int
			ci_case(SC,  a = *(char *)*sp++ = a;) // store char
			ci_case(PSH, *--sp = a;)              // push

			ci_case(OR,  a = *sp++ |  a;)
			ci_case(XOR, a = *sp++ ^  a;)
			ci_case(AND, a = *sp++ &  a;)
			ci_case(EQ,  a = *sp++ == a;)
			ci_case(NE,  a = *sp++ != a;)
			ci_case(LT,  a = *sp++ <  a;)
			ci_case(GT,  a = *sp++ >  a;)
			ci_case(LE,  a = *sp++ <= a;)
			ci_case(GE,  a = *sp++ >= a;)
			ci_case(SHL, a = *sp++ << a;)
			ci_case(SHR, a = *sp++ >> a;)
			ci_case(ADD, a = *sp++ +  a;)
			ci_case(SUB, a = *sp++ -  a;)
			ci_case(MUL, a = *sp++ *  a;)
			ci_case(DIV, a = *sp++ /  a;)
			ci_case(MOD, a = *sp++ %  a;)

			ci_case(OPEN, a = (int)fopen((const char *)sp[1], (const char *)*sp);)
			ci_case(READ, a = fread((char *)sp[3], sp[2], sp[1], (FILE *)*sp);)
			ci_case(CLOS, a = fclose((FILE *)*sp);)
			ci_case(PRTF,
		t = sp + pc[1];
		a = printf((char *)t[-1], t[-2], t[-3], t[-4], t[-5], t[-6]);
		)
			ci_case(MALC, a = (int)malloc(*sp);)
			ci_case(MSET, a = (int)memset((char *)sp[2], sp[1], *sp);)
			ci_case(MCMP, a = memcmp((char *)sp[2], (char *)sp[1], *sp);)
			ci_case(EXIT,
		printf(BLUE("exit(%d) cycle = %d\n"), *sp, cycle);
		return *sp;)

			ci_default(
		printf(RED("unknown instruction = %d! cycle = %d\n"), i, cycle);
		return -1;)
		}
	}
	return 0;
}

// vim: tabstop=4 shiftwidth=4 softtabstop=4