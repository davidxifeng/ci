#pragma once

#define COLOR_RED     "\x1b[31m"
#define COLOR_GREEN   "\x1b[32m"
#define COLOR_YELLOW  "\x1b[33m"
#define COLOR_BLUE    "\x1b[34m"
#define COLOR_RESET   "\x1b[0m"

#define RED(s)    (COLOR_RED    s COLOR_RESET)
#define GREEN(s)  (COLOR_GREEN  s COLOR_RESET)
#define YELLOW(s) (COLOR_YELLOW s COLOR_RESET)
#define BLUE(s)   (COLOR_BLUE   s COLOR_RESET)

// tokens and classes (operators last and in precedence order)
enum Token {
	// " ~ ; ! { } ( ) ] , :
	Num = 128, Fun, Sys, Glo, Loc, Id,

	// keywords
	Char, Else, Enum, If, Int, Return, While,

	// operators
	Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt, Le, Ge,
	Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak
};

// opcodes
enum Opcodes {
	LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LGB ,

	LEV ,

	LI  ,LC  ,SI  ,SC  ,PSH ,

	OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,
	SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,

	OPEN,READ,CLOS,PRTF,MALC,MSET,MCMP,EXIT
};

extern const char *op_codes;

// types
// 1. basic types
// 2. pointer types: basic type + n * ptr
enum { CHAR, INT, /*add new types here*/ PTR };

// identifier offsets (since we can't create an ident struct)
enum { Tk, Hash, Name, Class, Type, Val, HClass, HType, HVal, Idsz };

struct Identifier {
	enum Token tk;
	int hash;
	char * name;
	enum Token tokenClass; // num glo loc fun sys
	int type;
	int value; // 函数地址 立即数值 ...

	enum Token hTokenClass;
	int hType;
	int hValue;
};

int save_process(const char * file, int *e, int *be, char *data, char *bd, int *sym);
int run_process(int argc, char **argv, int debug);

int run_c(int argc, char **argv, int debug, int main_addr);

void next();
void expr(int lev);
void stmt();
int parse();

// vim: tabstop=2 shiftwidth=2 softtabstop=2
