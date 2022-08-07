#[derive(Debug, PartialEq)]
enum Keyword {
    Char,
    Else,
    Enum,
    If,
    Int,
    Return,
    While,
}

#[derive(Debug, PartialEq)]
enum Operator {
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
    Inc,
    Dec,
    Brak,
}

#[derive(Debug, PartialEq)]
enum Token {
    Num(i64),
    Id(String),
    Fun,
    Sys,
    Glo,
    Loc,
    Keyword(Keyword),
    Operator(Operator),
}

/// 词法分析状态
#[derive(Debug, PartialEq)]
struct TokenState<'a> {
    /// 未处理的字符串
    input: &'a str,
    /// 当前行号
    line: isize,
}

impl Iterator for TokenState<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();

        let tk: Token = Token::Num(1);

        let r = match chars.next() {
            None => None,
            Some(c) => match c {
                 c if c > '0' && c < '9' => {
			Some(Ok(tk))
		 }
                _ => Some(Err("unknown".to_string())),
            },
        };

	self.input = chars.as_str();
	r
    }
}

/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tl = vec![];
    let ts = TokenState {
        input,
        line: 1,
    };
    for r in ts {
        match r {
            Ok(tk) => tl.push(tk),
            Err(err) => return Err(err),
        }
    }
    Ok(tl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_demo_lex() {
        assert_eq!(lex("12"), Ok(vec![Token::Num(1), Token::Num(1)]));
    }
}
