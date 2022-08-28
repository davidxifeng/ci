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
	Return(ReturnStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStmt {
	pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	Const(Const),
	CondExpr(CondExpr), // BinOp(BinOp)
	AssignExpr(AssignExpr),
	CommaExpr(CommaExpr), // BinOp(BinOp)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondExpr {
	cond: Box<Expr>,
	left: Box<Expr>, // then, expr
	right: Box<Expr> // else, conditional expr
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct BinOp {
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
	Variable(VariableDeclaration),
	Function(FunctionDefinition),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationList {
	pub(crate) list: Vec<Declaration>,
}
