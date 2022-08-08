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
#[derive(Debug)]
struct TokenState<'a> {
    chars_iter: std::str::Chars<'a>,
    /// 当前行号
    line: isize,
}

use itertools::Itertools;

#[inline]
fn is_digit(c: &char) -> bool {
    *c >= '0' && *c <= '9'
}

#[inline]
fn is_id_initial_char(c: &char) -> bool {
    let c = *c;
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}
#[inline]
fn is_id_char(c: &char) -> bool {
    is_id_initial_char(c) || is_digit(c)
}

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
        let chars = &mut self.chars_iter;
        loop {
            match chars.next() {
                None => {
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
                    // 这里的做法太缺乏思考和学习了. 应该所有的标识符一起处理
                    // 然后根据 '符号表' 判断是关键字还是普通标识符
                    _ if is_id_initial_char(&c) => {
                        let mut ids = String::from(c);
                        while let Some(idc) = chars.peeking_take_while(is_id_char).next() {
                            ids.push(idc);
                        }
                        return Some(Token::Id(ids));
                    }
                    ch if is_digit(&c) => {
                        // as u8 or u32, which is better?
                        let mut iv = ch as u32 - '0' as u32;
                        while let Some(nch) = chars.peeking_take_while(is_digit).next() {
                            iv = iv * 10 + (nch as u32) - ('0' as u32);
                        }
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
    TokenState {
        chars_iter: input.chars(),
        line: 1,
    }
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lex_1() {
        assert_eq!(lex("123"), vec![Token::Num(123)]);
        assert_eq!(lex("1 23"), vec![Token::Num(1), Token::Num(23)]);
        assert_eq!(
            lex("1x23"),
            vec![Token::Num(1), Token::Id("x23".to_string())]
        );
    }

    #[test]
    fn run_lex_2() {
        assert_eq!(lex("if"), vec![Token::Id("if".to_string())]);
        assert_eq!(lex("ix"), vec![Token::Id("ix".to_string())]);
        assert_eq!(
            lex("if ix"),
            vec![Token::Id("if".to_string()), Token::Id("ix".to_string())]
        );
        assert_eq!(
            lex("else 123"),
            vec![Token::Id("else".to_string()), Token::Num(123)]
        );
        assert_eq!(lex("if_123"), vec![Token::Id("if_123".to_string())]);
    }

    #[test]
    fn test_put_back() {
	let mut c = itertools::put_back("hello".chars());
	c.put_back('X');
	c.put_back('Y'); // 会覆盖上一次,因为内部只有一个空间
	for v in c {
		println!("{}", v);
	}
    }

    #[test]
    fn test_put_back_n() {
	let mut c = itertools::put_back_n("hello".chars());
	c.put_back('Z');
	c.put_back('Y');
	c.put_back('X');
	for v in c {
		println!("{}", v);
	}
    }
}
