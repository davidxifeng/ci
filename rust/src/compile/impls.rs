use crate::lex::*;

use super::types::*;

impl std::fmt::Display for CiType {
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
			Self::Variable { ci_type, list } => {
				write!(f, "{}", ci_type)?;

				if !list.is_empty() {
					write!(f, " {}", list[0])?;
				}
				for v in list.iter().skip(1) {
					write!(f, ", ")?;
					write!(f, "{}", v)?;
				}
				Ok(())
			}
		}
	}
}

impl std::convert::From<Vec<Declaration>> for DeclarationList {
	fn from(l: Vec<Declaration>) -> Self {
		DeclarationList { list: (l) }
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
