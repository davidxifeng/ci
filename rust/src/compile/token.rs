#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Const(Const),
	StringLiteral(String),
	Keyword(Keyword),
	Id(String),
	Punct(Punct),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenList {
	pub data: Vec<Token>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
	Auto,
	Bool,
	Break,
	Case,
	Char,
	Complex,
	Const,
	Continue,
	Default,
	Do,
	Double,
	Else,
	Enum,
	Extern,
	Float,
	For,
	Goto,
	If,
	Imaginary,
	Inline,
	Int,
	Long,
	Register,
	Restrict,
	Return,
	Short,
	Signed,
	SizeOf,
	Static,
	Struct,
	Switch,
	Typedef,
	Union,
	Unsigned,
	Void,
	Volatile,
	While,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Punct {
	Comma,
	Assign,
	/// ?
	Cond,
	Lor,
	Lan,
	Or,
	Xor,
	/// &
	And,
	Eq,
	Ne,
	Lt,
	Gt,
	Le,
	Ge,
	Shl,
	Shr,
	Add,
	Sub,
	/// *
	Mul,
	Div,
	Mod,
	/// ++
	Inc,
	/// --
	Dec,
	/// [
	BrakL,
	/// ]
	BrakR,

	AssignAdd,
	AssignSub,
	AssignMul,
	AssignDiv,
	AssignMod,
	AssignShl,
	AssignShr,
	AssignBAnd,
	AssignBOr,
	AssignBXor,

	/// .
	Dot,
	/// ->
	Arrow,
	/// !
	Not,
	/// ;
	Semicolon,
	/// (
	ParentheseL,
	/// )
	ParentheseR,
	/// }
	BracesR,
	/// {
	BracesL,
	/// :
	Colon,
	/// ~
	Tilde,
	/// ...
	VARARG,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Const {
	#[default]
	Empty,
	Integer(String),
	Character(char),
}
