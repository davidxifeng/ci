#include <stdio.h>
#include <stdlib.h>

#include "ci.h"

extern char *p, *lp;
extern char *data;

extern int src, line, *le, *e, ival, *id, *sym;
extern enum Token tk;

extern int loc; // local variable offset
extern int ty;  // current expression type

void parse_expr() {
    int t, *d;
    switch ((int)tk) {
        case 0:
            printf("%d: unexpected eof in expression\n", line);
            exit(-1);
            break;
        case Num:
            *++e = IMM; *++e = ival;
            next(); ty = INT;
            break;
        case '"':
            *++e = IMM; *++e = ival; next();
            while (tk == '"') next();
            // align data pointer by 4 and terminate string with zero
            // (memory block is init with zero)
            data = (char *)((int)data + 4 & -4); ty = PTR;
            break;
        case Id:
            d = id; next();
            // function call
            if (tk == '(') {
                next();
                t = 0;
                while (tk != ')') {
                    // 参数push
                    expr(Assign);
                    *++e = PSH; ++t;
                    if (tk == ',') next();
                }

                next();

                if (d[Class] == Sys) {
                    *++e = d[Val];
                } else if (d[Class] == Fun) {
                    *++e = JSR; *++e = d[Val];
                } else {
                    printf("%d: bad function call\n", line); exit(-1);
                }
                if (t) {
                    *++e = ADJ;
                    *++e = t;
                }
                ty = d[Type]; // 表达式类型 == 函数的返回值类型
            } else {

                // 枚举值
                if (d[Class] == Num) {
                    *++e = IMM; *++e = d[Val];
                    ty = INT;
                } else {
                    // 变量
                    if (d[Class] == Loc) {
                        *++e = LEA;
                        *++e = loc - d[Val]; // local variable value: offset
                    } else if (d[Class] == Glo) {
                        *++e = LGB;
                        *++e = d[Val];
                    } else {
                        printf("%d: undefined variable\n", line); exit(-1);
                    }
                    *++e = ((ty = d[Type]) == CHAR) ? LC : LI;
                }
            }
            break;
        case '(':
            // 括号
            // 1. 强制类型转换
            // 下一个是类型token (Int, Char)
            // 2. 括号表达式
            next();
            if (tk == Int || tk == Char) {
                t = (tk == Int) ? INT : CHAR;

                next();
                // pointer type process
                while (tk == Mul) {
                    next(); t = t + PTR;
                }
                if (tk == ')') {
                    next();
                } else {
                    printf("%d: bad cast\n", line); exit(-1);
                }
                expr(Inc);
                ty = t;
            } else {
                expr(Assign);
                if (tk == ')')  {
                    next();
                } else {
                    printf("%d: close paren expected\n", line); exit(-1);
                }
            }
            break;
        case Mul:
            // 指针解引用
            next();
            expr(Inc);
            // check point type
            if (ty > INT) {
                ty = ty - PTR; // 脱掉一层指针
            } else {
                printf("%d: bad dereference\n", line); exit(-1);
            }
            *++e = (ty == CHAR) ? LC : LI;
            break;
        case And:
            // 取地址操作
            // 要求下一个表达式的值一定是局部变量或全局变量, 通过load指令来判断
            // 如果是的话就删除load指令,这样就刚好地址在寄存器中了
            next(); expr(Inc);
            if (*e == LC || *e == LI) {
                --e;
            } else {
                printf("%d: bad address-of\n", line); exit(-1);
            }
            ty = ty + PTR;
            break;
        case '!':
            // 位运算 取反
            next();
            expr(Inc); *++e = PSH;
            *++e = IMM; *++e = 0;
            *++e = EQ;
            ty = INT;
            break;
        case '~':
            // 位运算 异或
            next();
            expr(Inc); *++e = PSH;
            *++e = IMM; *++e = -1;
            *++e = XOR;
            ty = INT;
            break;
        case Add:
            // 一元运算符+
            next(); expr(Inc); ty = INT;
            break;
        case Sub:
            next();
            // 负数 立即数直接计算, 变量的话 * -1
            *++e = IMM;
            if (tk == Num) {
                *++e = -ival; next();
            } else {
                *++e = -1;
                *++e = PSH;
                expr(Inc);
                *++e = MUL;
            }
            ty = INT;
            break;
        case Inc:
        case Dec:
            // 指针的自增 自减表达式
            t = tk; next();
            expr(Inc);
            // 变量表达式, 其最后一条指令是Load, 然后下面的操作是修改生成的指令
            // 计算地址修改
            if (*e == LC) { *e = PSH; *++e = LC; }
            else if (*e == LI) { *e = PSH; *++e = LI; }
            else { printf("%d: bad lvalue in pre-increment\n", line); exit(-1); }
            *++e = PSH;
            *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
            *++e = (t == Inc) ? ADD : SUB;
            *++e = (ty == CHAR) ? SC : SI;
            break;
        default:
            printf("%d: bad expression\n", line);
            exit(-1);
    }
}

void expr(int lev) {
    int t, *d;

    parse_expr();

    // "precedence climbing" or "Top Down Operator Precedence" method
    while (tk >= lev) {
        t = ty;
        if (tk == Assign) {
            next();
            if (*e == LC || *e == LI) {
                *e = PSH; // 左值是变量, 修改其load指令
            } else {
                printf("%d: bad lvalue in assignment\n", line); exit(-1);
            }
            expr(Assign);// 计算右值
            *++e = ((ty = t) == CHAR) ? SC : SI; // 增加save指令
            // 赋值表达式 可以多次赋值: 如 i = j = 3;
            // 考虑增加一个 a, b = 1, 2的扩展玩
        } else if (tk == Cond) {
            next();
            *++e = BZ; d = ++e;
            expr(Assign);
            if (tk == ':') {
                next();
            } else {
                printf("%d: conditional missing colon\n", line); exit(-1);
            }
            *d = (int)(e + 3 - d);

            *++e = JMP;
            d = ++e;
            expr(Cond);
            *d = (int)(e + 1 - d);
        }
        else if (tk == Lor) {
            next();
            *++e = BNZ; d = ++e;
            expr(Lan);
            *d = (int)(e + 1 - d);
            ty = INT;
        }
        else if (tk == Lan) {
            next();
            *++e = BZ;  d = ++e;
            expr(Or);
            *d = (int)(e + 1 - d);
            ty = INT;
        }
        else if (tk == Or)  { next(); *++e = PSH; expr(Xor); *++e = OR;  ty = INT; }
        else if (tk == Xor) { next(); *++e = PSH; expr(And); *++e = XOR; ty = INT; }
        else if (tk == And) { next(); *++e = PSH; expr(Eq);  *++e = AND; ty = INT; }
        else if (tk == Eq)  { next(); *++e = PSH; expr(Lt);  *++e = EQ;  ty = INT; }
        else if (tk == Ne)  { next(); *++e = PSH; expr(Lt);  *++e = NE;  ty = INT; }
        else if (tk == Lt)  { next(); *++e = PSH; expr(Shl); *++e = LT;  ty = INT; }
        else if (tk == Gt)  { next(); *++e = PSH; expr(Shl); *++e = GT;  ty = INT; }
        else if (tk == Le)  { next(); *++e = PSH; expr(Shl); *++e = LE;  ty = INT; }
        else if (tk == Ge)  { next(); *++e = PSH; expr(Shl); *++e = GE;  ty = INT; }
        else if (tk == Shl) { next(); *++e = PSH; expr(Add); *++e = SHL; ty = INT; }
        else if (tk == Shr) { next(); *++e = PSH; expr(Add); *++e = SHR; ty = INT; }
        else if (tk == Add) {
            next(); *++e = PSH; expr(Mul);
            if ((ty = t) > PTR) { *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;  }
            *++e = ADD;
        }
        else if (tk == Sub) {
            next();
            *++e = PSH; expr(Mul);
            if ((ty = t) > PTR) {
                *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;
            }
            *++e = SUB;
        }
        else if (tk == Mul) { next(); *++e = PSH; expr(Inc); *++e = MUL; ty = INT; }
        else if (tk == Div) { next(); *++e = PSH; expr(Inc); *++e = DIV; ty = INT; }
        else if (tk == Mod) { next(); *++e = PSH; expr(Inc); *++e = MOD; ty = INT; }
        else if (tk == Inc || tk == Dec) {
            if (*e == LC) { *e = PSH; *++e = LC; }
            else if (*e == LI) { *e = PSH; *++e = LI; }
            else { printf("%d: bad lvalue in post-increment\n", line); exit(-1); }
            *++e = PSH; *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
            *++e = (tk == Inc) ? ADD : SUB;
            *++e = (ty == CHAR) ? SC : SI;
            *++e = PSH; *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
            *++e = (tk == Inc) ? SUB : ADD;
            next();
        }
        else if (tk == Brak) {
            next(); *++e = PSH; expr(Assign);
            if (tk == ']') {
                next();
            } else {
                printf("%d: close bracket expected\n", line); exit(-1);
            }

            if (t > PTR) {
                *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;
            }
            else if (t < PTR) {
                printf("%d: pointer type expected\n", line); exit(-1);
            }
            *++e = ADD;
            *++e = ((ty = t - PTR) == CHAR) ? LC : LI;
        }
        else {
            printf("%d: compiler error tk=%d\n", line, tk); exit(-1);
        }
    }
}

