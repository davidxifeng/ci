use super::token::{Const, Punct};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
	Variable(Variable),
	Function(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
	pub next: Option<Box<Object>>,
	pub name: Option<String>,
	pub ctype: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
	pub next: Option<Box<Object>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Void,
	Bool,
	Char,
	// Short(Short),
	Int,
	// Long(Long),
	// Float(Float),
	// Double(Double),
	Ptr(Ptr),
	Array(Array),
	// Enum(Enum),
	// Struct(Struct),
	// Union(Union),
	Func(Func),
}

pub const TYPE_VOID: Type = Type::Void;
pub const TYPE_BOOL: Type = Type::Bool;
pub const TYPE_CHAR: Type = Type::Char;
pub const TYPE_INT: Type = Type::Int;

impl Type {
	pub fn to_pointer(self) -> Self {
		Type::Ptr(Ptr { base_type: Box::new(self) })
	}

	pub fn to_array(self, expr: Option<Expr>) -> Self {
		let length = match expr {
			Some(Expr::Const(Const::Integer(ref i))) => i.parse().unwrap(),
			_ => 0,
		};
		Type::Array(Array { base_type: Box::new(self), length, size_expr: expr })
	}
}

pub fn avoid_warnings() {}

pub trait TypeSizeAlign {
	fn size(&self) -> usize;
	fn align(&self) -> usize;
}

impl TypeSizeAlign for Type {
	fn size(&self) -> usize {
		match self {
			Self::Void => 0,
			Self::Bool => 1,
			Self::Char => 1,
			Self::Int => 4,
			Self::Array(Array { base_type, length, size_expr: _ }) => base_type.size() * length,
			Self::Ptr(_) => 8,
			Self::Func(_) => 8,
		}
	}
	fn align(&self) -> usize {
		match self {
			Self::Void => 1,
			Self::Bool => 1,
			Self::Char => 1,
			Self::Int => 4,
			Self::Array(Array { base_type, length: _, size_expr: _ }) => base_type.align(),
			Self::Ptr(_) => 8,
			Self::Func(_) => 8,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ptr {
	pub base_type: Box<Type>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
	pub length: usize,
	pub size_expr: Option<Expr>,
	pub base_type: Box<Type>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
	pub return_type: Box<Type>,
	pub param_list: Vec<Type>,
	pub is_variadic: bool,
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
