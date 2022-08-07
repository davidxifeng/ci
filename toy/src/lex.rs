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

use itertools::Itertools;

impl Iterator for TokenState<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();

        loop {
            match chars.next() {
                None => {
                    self.input = chars.as_str();
                    return None;
                }
                Some(c) => match c {
                    ch if ch > '0' && ch < '9' => {
                        // as u8 or u32, which is better?
                        let mut iv = ch as u32 - '0' as u32;
                        while let Some(nch) =
                            chars.peeking_take_while(|&x| x >= '0' && x <= '9').next()
                        {
                            iv = iv * 10 + (nch as u32) - ('0' as u32);
                        }
                        self.input = chars.as_str();
                        return Some(Token::Num(iv as i64));
                    }
                    _ => {}
                },
            };
        }
    }
}

/// 对输入字符串进行词法解析,得到一组token list,或者错误信息
fn lex(input: &str) -> Vec<Token> {
    TokenState { input, line: 1 }.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_demo_lex() {
        assert_eq!(lex("123"), vec![Token::Num(123)]);
        assert_eq!(lex("1 23"), vec![Token::Num(1), Token::Num(23)]);
        assert_eq!(lex("1x23"), vec![Token::Num(1), Token::Num(23)]);
    }
}
