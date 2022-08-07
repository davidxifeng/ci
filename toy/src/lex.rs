#[derive(Debug, PartialEq)]
enum Keyword {
    Char,
    Int,
    Enum,
    If,
    Else,
    While,
    Return,
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
    /// 或许需要调整一下
    Keyword(Keyword),
    Unknown(char),
    Id(String),
    Fun,
    Sys,
    Glo,
    Loc,
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

    /// 当前结构果然思路上还有严重的问题,状态不足,不能识别出词法阶段的错误
    /// 词法识别阶段确实也可以检测出错误,比如 数字后面只能接空白 或者是运算符,不能是数字
    /// 识别关键字的时候,好像还需要回退: 比如识别的i后,如果后面不是f,必须当作其他关键字或普通
    /// 标识符
    /// c4中的做法: 提前准备好符号表,把关键字 还有库函数添加到符号表中,
    /// 然后next函数只识别标识符, 并不区分 关键字 还是库函数,或者普通变量
    /// 文档上看到说go语言解析可以不用符号表,不知道是什么意思.
    /// 只有25个关键字, 不知是不是和词法解析有关系. 关键字和预定义标识符等内在关系上面
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();

        loop {
            match chars.next() {
                None => {
                    self.input = chars.as_str();
                    return None;
                }
                Some(c) => match c {
                    '\r' => {
                        chars.peeking_take_while(|&x| x == '\n').next();
                        self.line += 1;
                    }
                    '\n' => {
                        self.line += 1;
                    }
                    'i' => {
                        if let Some(_) = chars.peeking_take_while(|&x| x == 'f').next() {
                            self.input = chars.as_str();
                            return Some(Token::Keyword(Keyword::If));
                        } else {
                            self.input = chars.as_str();
                            return Some(Token::Unknown(c));
                        }
                    }
                    'e' => {
                        // Monad的感觉
                        if let Some(_) = chars
                            .peeking_take_while(|&x| x == 'l')
                            .next()
                            .and_then(|_| chars.peeking_take_while(|&x| x == 's').next())
                            .and_then(|_| chars.peeking_take_while(|&x| x == 'e').next())
                        {
                            self.input = chars.as_str();
                            return Some(Token::Keyword(Keyword::Else));
                        } else {
                            self.input = chars.as_str();
                            return Some(Token::Unknown(c));
                        }
                    }
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
    fn run_lex_1() {
        assert_eq!(lex("123"), vec![Token::Num(123)]);
        assert_eq!(lex("1 23"), vec![Token::Num(1), Token::Num(23)]);
        assert_eq!(lex("1x23"), vec![Token::Num(1), Token::Num(23)]);
    }

    #[test]
    fn run_lex_2() {
        assert_eq!(lex("if"), vec![Token::Keyword(Keyword::If)]);
        assert_eq!(lex("ix"), vec![Token::Unknown('i')]);
        assert_eq!(
            lex("ifix"),
            vec![Token::Keyword(Keyword::If), Token::Unknown('i')]
        );
        assert_eq!(
            lex("ifelse"),
            vec![Token::Keyword(Keyword::If), Token::Keyword(Keyword::Else)]
        );
        assert_eq!(
            lex("if123"),
            vec![Token::Keyword(Keyword::If), Token::Num(123)]
        );
    }
}
