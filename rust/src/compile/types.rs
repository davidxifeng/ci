use super::token::{Keyword, Const};

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
	// BinOp(BinOp)
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
