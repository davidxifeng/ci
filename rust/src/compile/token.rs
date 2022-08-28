#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Const(Const),
	StringLiteral(String),
	Keyword(Keyword),
	Id(String),
	Punct(Punct),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
	Char,
	Int,
	Enum,

	If,
	Else,
	While,
	Return,
}

// Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt, Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Punct {
	/// = 优先级顺序开始 这块的符号顺序表示优先级,ordering matters
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
	/// !
	Not,
	/// ;
	Semicolon,
	/// ,
	Comma,
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

