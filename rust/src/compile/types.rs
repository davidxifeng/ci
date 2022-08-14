use crate::lex::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiType {
	BaseType(Keyword),
	// CiEnum(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarator {
	pub name: String,
	pub value: Const,
	// idr: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
	Variable { ci_type: CiType, list: Vec<Declarator> },
	// Function { ci_type: CiType, name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationList {
	pub(crate) list: Vec<Declaration>,
}
