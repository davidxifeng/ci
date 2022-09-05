use super::token::{Const, Punct};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarator {
	pub name: String,
	pub value: Const,
	// idr: i32,
}

/// 变量定义+初始化
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration {
	pub list: Vec<Declarator>,
}

/// 函数定义
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition {
	/// 函数名
	pub name: String,

	pub params: Vec<Parameter>,

	pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Statement {
	#[default]
	Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
	Const(Const),
	Id(String),
	StringLiteral(String),

	MemberAccess(Box<Expr>, String),
	MemberAccessP(Box<Expr>, String),
	Postfix(PostfixOP),
	FunctionCall(Box<Expr>, Vec<Expr>),

	UnaryOp(UnaryOp),
	BinOp(BinOp),
	CondExpr(CondExpr),
	AssignExpr(AssignExpr),
	CommaExpr(CommaExpr),
}

impl Expr {
	pub fn new_member_access(expr: Self, id: String) -> Self {
		Expr::MemberAccess(Box::new(expr), id)
	}

	pub fn new_member_access_p(expr: Self, id: String) -> Self {
		Expr::MemberAccessP(Box::new(expr), id)
	}

	pub fn new_func_call(expr: Self, args: Vec<Expr>) -> Self {
		Expr::FunctionCall(Box::new(expr), args)
	}

	pub fn new_assign(left: Self, op: Punct, right: Self) -> Self {
		Expr::AssignExpr(AssignExpr { left: Box::new(left), assign: op, right: Box::new(right) })
	}

	pub fn new_binary(left: Self, op: Punct, right: Self) -> Self {
		Expr::BinOp(BinOp { left: Box::new(left), op, right: Box::new(right) })
	}

	pub fn new_comma(left: Self, right: Self) -> Self {
		Expr::CommaExpr(CommaExpr { left: Box::new(left), right: Box::new(right) })
	}

	pub fn new_cond(cond: Self, left: Self, right: Self) -> Self {
		Expr::CondExpr(CondExpr { cond: Box::new(cond), left: Box::new(left), right: Box::new(right) })
	}

	pub fn new_unary(op: Punct, expr: Self) -> Self {
		Expr::UnaryOp(UnaryOp { op, expr: Box::new(expr) })
	}

	pub fn new_postfix(op: Punct, expr: Self) -> Self {
		Expr::Postfix(PostfixOP { op, expr: Box::new(expr) })
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostfixOP {
	pub op: Punct,
	pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOp {
	pub op: Punct,
	pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOp {
	pub left: Box<Expr>,
	pub op: Punct,
	pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondExpr {
	pub cond: Box<Expr>,
	pub left: Box<Expr>,  // then, expr
	pub right: Box<Expr>, // else, conditional expr
}
// C语法中, x ? a : b = 2 被解释为:
// (x ? a : b) = 2, 而C++中则为 (x ? a : (b = 2)), 更最小惊讶.
// 条件表达式并不是左值,所以C中为非法语句.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
	pub left: Box<Expr>,
	pub assign: Punct, // = += -= *= ...
	pub right: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationList {
	pub list: Vec<Declaration>,
}
