use crate::lex::*;

use super::types::*;

impl std::fmt::Display for CType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::BaseType(kw) => match kw {
				Keyword::Char => "char",
				Keyword::Int => "int",
				_ => "<error>",
			},
		})
	}
}

impl std::fmt::Display for Declarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value {
			Const::Empty => write!(f, "{}", self.name),
			_ => write!(f, "{} = {}", self.name, self.value),
		}
	}
}

impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Variable(v) => {
				write!(f, "{}", v.ctype)?;

				if !v.list.is_empty() {
					write!(f, " {}", v.list[0])?;
				}
				for v in v.list.iter().skip(1) {
					write!(f, ", ")?;
					write!(f, "{}", v)?;
				}
				Ok(())
			}
			Self::Function(_) => Ok(()),
		}
	}
}

impl std::fmt::Display for DeclarationList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for v in &self.list {
			writeln!(f, "{};", v)?;
		}
		Ok(())
	}
}

impl std::convert::From<Vec<Declaration>> for DeclarationList {
	fn from(l: Vec<Declaration>) -> Self {
		DeclarationList { list: (l) }
	}
}
