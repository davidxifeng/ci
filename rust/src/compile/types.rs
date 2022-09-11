use super::token::{Const, Punct};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
	Variable(Variable),
	Function(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
	pub name: String,
	pub ctype: Type,
	pub init_value: Option<Expr>,

	pub is_local: bool,
	pub is_tentative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
	pub name: String,
	pub ctype: Func,
	pub locals: Vec<Object>,
	pub stmts: Statement,
	pub stack_size: usize,
	pub is_definition: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Void,
	Bool,
	Char,
	Int,
	Ptr(Ptr),
	Array(Array),
	Func(Func),
	// Enum(Enum), // Struct(Struct), // Union(Union),
	// Short(Short), // Long(Long), // Float(Float), // Double(Double),
}

pub const TYPE_VOID: Type = Type::Void;
pub const TYPE_BOOL: Type = Type::Bool;
pub const TYPE_CHAR: Type = Type::Char;
pub const TYPE_INT: Type = Type::Int;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeIdentifier {
	pub name: Option<String>,
	pub ctype: Type,
}

impl TypeIdentifier {
	pub fn new(ctype: Type, name: Option<String>) -> Self {
		TypeIdentifier { name, ctype }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VarAttr {
	pub is_typedef: bool,
	pub is_static: bool,
	pub is_extern: bool,
	pub is_inline: bool,
}

impl Type {
	pub fn get_func(&self) -> Option<Func> {
		match self {
			Type::Func(f) => Some(f.clone()),
			_ => None,
		}
	}

	pub fn into_pointer(self) -> Self {
		Type::Ptr(Ptr { base_type: Box::new(self) })
	}

	pub fn into_array(self, expr: Option<Expr>) -> Self {
		let length = match expr {
			Some(Expr::Const(Const::Integer(ref i))) => i.parse().unwrap_or_default(),
			_ => 0,
		};
		Type::Array(Array { base_type: Box::new(self), length, size_expr: expr })
	}

	pub fn into_function(self) -> Self {
		Type::Func(Func { return_type: Box::new(self), param_list: vec![], is_variadic: false })
	}

	pub fn into_function_with_param(self, param_list: Vec<TypeIdentifier>) -> Self {
		Type::Func(Func { return_type: Box::new(self), param_list, is_variadic: false })
	}
}

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
	pub param_list: Vec<TypeIdentifier>,
	pub is_variadic: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
	Empty,
	ExprStmt(Expr),
	ReturnStmt(Expr),
	CompoundStmt(Vec<Statement>),
	IfStmt(Expr, Box<Statement>, Option<Box<Statement>>),
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
