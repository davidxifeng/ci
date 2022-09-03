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
	pub token_list: Vec<Token>,
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

// Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt, Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Punct {
	/// = 优先级顺序开始 这块的符号顺序表示优先级,ordering matters
	/// ,
	Comma,
	Assign,
	Cond,
	Lor,
	Lan,
	Or,
	Xor,
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
	Mul,
	Div,
	Mod,
	/// ++
	Inc,
	/// --
	Dec,
	/// [
	BrakL,
	/// 优先级顺序结束
	AssignAdd,
	AssignSub,
	AssignMul,
	AssignDiv,
	AssignMod,

	/// !
	Not,
	/// ;
	Semicolon,
	/// (
	ParentheseL,
	/// )
	ParentheseR,
	/// ]
	BrakR,
	/// }
	BracesR,
	/// {
	BracesL,
	/// :
	Colon,
	/// ~
	Tilde,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Const {
	#[default]
	Empty,
	Integer(String),
	Character(char),
}
