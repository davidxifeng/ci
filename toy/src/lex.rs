#[derive(Debug, PartialEq)]
enum Number {
    Unsigned(u64),
    Signed(i64),
}

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
    Num(Number),
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
struct TokenState <'a>{
    /// 处理的字符串
    input: &'a str,
    /// 当前处理到的位置
    index: usize,
    /// 当前行号
    line: isize,
}

impl Iterator for TokenState<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            None
        } else {
            self.index += 1;
            Some(Ok(Token::Fun))
        }
    }
}

/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tl = vec![];
    let ts = TokenState {
        input,
        index: 0,
        line: 1,
    };
    for r in ts {
        match r {
            Ok(tk) => {
                tl.push(tk);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(tl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_demo_lex() {
        assert_eq!(lex("hi"), Ok(vec![Token::Fun, Token::Fun]));
    }
}
