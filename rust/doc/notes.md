# 笔记

## 编译器中的树:

* Binary expression tree

```
            ┌───5
        ┌───^
        │   └───4
    ┌───*
    │   └───3
────+
    │   ┌───2
    └───*
        └───1
```

* 解析树
* 抽象语法树 AST


## C语言中的表达式 (6.5 Expressions)

### overview

An expression is a sequence of operators and operands that specifies computation of a
value, or that designates an object or a function, or that generates side effects, or that
performs a combination thereof.

表达式是:

- 指定了值的计算的运算符和操作数的序列
- 表示(?)对象或者函数
- 产生副作用
- 以上的组合

designates: 指定 划定 委派
thereof: 其中 关于

Between the previous and next sequence point an object shall have its stored value
modified at most once by the evaluation of an expression.

在前后序列点之间, 对象存储的值应该在表达式求值的过程中只修改一次.

Furthermore, the prior value shall be read only to determine the value to be stored.

> This paragraph renders undefined statement expressions such as
>	```c
>	i=++i + 1; // 未定义行为
>	a[i++] = i; // 未定义行为
>	```
> while allowing
>	```c
>	i=i+1;
>	a[i] = i;
>	```

The grouping of operators and operands is indicated by the syntax.

运算的分组由语法指定. (通过语法指定了运算符在表达式求值中的优先级. 以及左结合 右结合等内容,
实际上C规范中确实没有运算符优先级的定义,而是通过语法确定的.)

Except as specified later (for the function-call (), &&, ||, ?:, and comma operators),
the order of evaluation of subexpressions and the order in which side effects
take place are both unspecified.

除了稍后规定的(函数调用, && || ?: ,)子表达式的求值顺序, 副作用生效的顺序未指定.

Some operators (the unary operator ~(取反), and the binary operators <<, >>, &, ^(异或), and |,
collectively described as **bitwise operators**) are required to have operands that have
integer type. These operators yield values that depend on the internal representations of
integers, and have implementation-defined and undefined aspects for signed types.

collectively: 统称,集体

位运算要求操作数是 整数, 对有符号类型而言有实现定义和未定义的行为. (是指有符号数的算术位移逻辑位移等行为?)

If an exceptional condition occurs during the evaluation of an expression (that is, if the
result is not mathematically defined or not in the range of representable values for its
type), the behavior is undefined.

异常发生,行为未定义.

The effective type of an object for an access to its stored value is the declared type of the
object, if any If a value is stored into an object having no declared type through an
lvalue having a type that is not a character type, then the type of the lvalue becomes the
effective type of the object for that access and for subsequent accesses that do not modify
the stored value. If a value is copied into an object having no declared type using
memcpy or memmove, or is copied as an array of character type, then the effective type
of the modified object for that access and for subsequent accesses that do not modify the
value is the effective type of the object from which the value is copied, if it has one. For
all other accesses to an object having no declared type, the effective type of the object is
simply the type of the lvalue used for the access.

有效类型,左值.(?)

An object shall have its stored value accessed only by an lvalue expression that has one of
the following types:

— a type compatible with the effective type of the object,
— a qualified version of a type compatible with the effective type of the object,
— a type that is the signed or unsigned type corresponding to the effective type of the
object,
— a type that is the signed or unsigned type corresponding to a qualified version of the
effective type of the object,
— an aggregate or union type that includes one of the aforementioned types among its
members (including, recursively,amember of a subaggregate or contained union), or
— a character type.

A floating expression may be contracted, that is, evaluated as though it were an atomic
operation, thereby omitting rounding errors implied by the source code and the
expression evaluation method. The FP_CONTRACT pragma in <math.h> provides a
way to disallow contracted expressions. Otherwise, whether and how expressions are
contracted is implementation-defined.

### 主表达式 6.5.1 Primary expressions

>	```
>	primary-expression: 只被 后缀运算符引用
>		identifier
>		constant
>		string-literal
>		( expression )
>	```

标识符 是变量时代表一个对象, 是左值 lvalue; 或者函数代号(function designator).
(所以,未声明的标识符违反了语法)

常量表达式

字符串字面量

parenthesized expression 也是主表达式. 类型和值与未括起来的表达式一样. 可能是 左值,函数代号
或者空表达式.


### 6.5.2 Postfix operators 后缀操作符

>	```
>	postfix-expression:  只被一元运算符引用
>		primary-expression
>		postfix-expression [ expression ] // 数组下标
>		postfix-expression ( argument-expression-listopt ) // 函数调用
>		postfix-expression . identifier 结构体和联合体成员
>		postfix-expression -> identifier
>		postfix-expression ++
>		postfix-expression --
>		( type-name ) { initializer-list } // 符合字面量
>		( type-name ) { initializer-list , }
>		argument-expression-list:
>		assignment-expression
>		argument-expression-list , assignment-expression
>	```

数组下标, `E1[E2]` == `(*((E1)+(E2)))`

函数调用

### 6.5.3 Unary operators 一元操作符

>	```
>	unary-expression:  被cast运算符 和 赋值运算符 引用
>		postfix-expression
>		++ unary-expression
>		-- unary-expression
>		unary-operator cast-expression
>		sizeof unary-expression
>		sizeof ( type-name )
>
>	unary-operator: one of
>		& * + - ~ !
>	```

6.5.3.4 sizeof: 不能用于函数或不完全类型

### 6.5.4 Cast operators

>	```
>	1 cast-expression:
>		unary-expression
>		( type-name ) cast-expression
>	```

### 6.5.5 Multiplicative operators

>	```
>	multiplicative-expression:
>		cast-expression
>		multiplicative-expression * cast-expression
>		multiplicative-expression / cast-expression
>		multiplicative-expression % cast-expression
>	```

### 接下来还有:

加 结合 运算符

移位运算符

关系运算符 > >= < <=

相等运算符 == !=

bit wise and运算符

>	```
>	AND-expression:
>		equality-expression
>		AND-expression & equality-expression
>	```

6.5.11 Bitwise exclusive OR operator

>	```
>	exclusive-OR-expression:
>		AND-expression
>		exclusive-OR-expression ^ AND-expression
>	```

6.5.12 Bitwise inclusive OR operator

逻辑与

逻辑或

6.5.15 Conditional operator

>	```
>	conditional-expression:
>		logical-OR-expression
>		logical-OR-expression ? expression : conditional-expression
>	```


### 6.5.16 Assignment operators

>	```
>	assignment-expression:
>		conditional-expression
>		unary-expression assignment-operator assignment-expression
>		assignment-operator: one of
>		= *= /= %= += -= <<= >>= &= ^= |=
>	```

所有赋值表达式都要求可修改的左值作为左 operand

简单赋值表达式, 符合赋值表达式.

### 6.5.17 Comma operator 优先级最低的运算符

表达式树的root

>	```
>	expression:
>		assignment-expression
>		expression , assignment-expression
>	```

`f(a, (t=3, t+2), c)` 正如语法所指示的一样, 逗号运算符不能出现在逗号用于分割list的上下文.
但可以用()表达式, 或者作为条件运算符的第二个表达式. `f(a, ok ? t = 0, t+2: c)`.

逗号运算符不产生左值.

### 6.6 Constant expressions

>	```
>	constant-expression:
>		conditional-expression
>	```

编译时而不是运行时求值.

Constant expressions shall not contain assignment, increment, decrement, function-call,
or comma operators, except when they are contained within a subexpression that is not
evaluated.

产生的值应该在其类型范围之内.

以上就是全部的表达式了.

## 语法

表达式 声明 语句

声明: 定义变量, 定义函数

>	```
>	(6.8) statement:
>		labeled-statement
>		compound-statement
>		expression-statement
>		selection-statement
>		iteration-statement
>		jump-statement
>	```

标签语句:

>	```
>	labeled-statement:
>		identifier : statement
>		case constant-expression : statement
>		default : statement
>	```

表达式语句

选择语句包括 if/ if/else/ switch

迭代语句

>	```
>	iteration-statement:
>		while ( expression ) statement
>		do statement while ( expression ) ;
>		for ( expression opt ; expression opt ; expression opt ) statement
>		for ( declaration expression <sub>opt</sub> ; expression <sub>opt</sub> ) statement
>	```

(不明白为何这样少了一个 ; expression <sub>opt</sub>, 或许只是不严谨?)

`for ( clause-1 ; expression-2 ; expression-3 ) statement`
1和3都可以省略. 表达式2省略的话,会被替换成非零的常量.

注意两种for语句分开定义了

跳转语句

>	```
>	jump-statement:
>		goto identifier ;
>		continue ;
>		break ;
>		return expressionopt ;
>	```