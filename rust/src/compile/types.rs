use super::token::{Const, Keyword, Punct};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CType {
	BaseType(Keyword),
	// CiEnum(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarator {
	pub name: String,
	pub value: Const,
	// idr: i32,
}

/// 变量定义+初始化
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration {
	pub ctype: CType,
	pub list: Vec<Declarator>,
}

/// 函数定义
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition {
	/// 函数返回值类型
	pub ctype: CType,

	/// 函数名
	pub name: String,

	pub params: Vec<Parameter>,

	pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
	pub ctype: CType,
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Statement {
	#[default]
	Empty,
	ReturnStmt(Expr),
	ExprStmt(Expr)
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	Const(Const),
	StringLiteral(String),
	Id(String),
	SimplePostfix(PostfixOP),
	UnaryOp(UnaryOp),
	BinOp(BinOp),
	CondExpr(CondExpr),
	AssignExpr(AssignExpr),
	CommaExpr(CommaExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct PostfixOP {
	op: Punct,
	expr: Box<Expr>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOp {
	op: Punct,
	expr: Box<Expr>
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp {
	left: Box<Expr>,
	op: Punct,
	right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondExpr {
	cond: Box<Expr>,
	left: Box<Expr>,  // then, expr
	right: Box<Expr>, // else, conditional expr
}
// C语法中, x ? a : b = 2 被解释为:
// (x ? a : b) = 2, 而C++中则为 (x ? a : (b = 2)), 更最小惊讶.
// 条件表达式并不是左值,所以C中为非法语句.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
	left: Box<Expr>,
	assign: Punct, // = += -= *= ...
	right: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaExpr {
	pub expr: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
	Variable(VariableDeclaration),
	Function(FunctionDefinition),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationList {
	pub(crate) list: Vec<Declaration>,
}
