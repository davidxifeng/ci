#pragma once

#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_COLOR_YELLOW  "\x1b[33m"
#define ANSI_COLOR_BLUE    "\x1b[34m"
#define ANSI_COLOR_RESET   "\x1b[0m"

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

struct Process {
    int * be;
    int text_size;
    char * bd;
    int data_size;
    int main_addr;
};

struct Process * create_process(int * e, int * be, char * data, char * bd, int * sym);
struct Process * load_process(const char * process_file);
int save_process(const char * process_file, struct Process * p);
void free_process(struct Process * process);
int run_process(int argc, char **argv, int debug);

int run_c(int argc, char **argv, int debug, int main_addr);
void next();
void expr(int lev);
void stmt();
int parse();

extern const char *op_codes;

// vim: tabstop=4 shiftwidth=4 softtabstop=4
