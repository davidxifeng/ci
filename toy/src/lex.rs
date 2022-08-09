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
enum Punct {
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
pub enum Token {
    IntegerConst(i64),
    CharacterConst(char),
    StringLiteral(String),
    Keyword(Keyword),
    Id(String),
    Punct(Punct),
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidChar(char),
}

pub struct TokenApi {}

impl TokenApi {
    /// 当前结构果然思路上还有严重的问题,状态不足,不能识别出词法阶段的错误
    /// 词法识别阶段确实也可以检测出错误,比如 数字后面只能接空白 或者是运算符,不能是数字
    /// 识别关键字的时候,好像还需要回退: 比如识别的i后,如果后面不是f,必须当作其他关键字或普通
    /// 标识符
    /// c4中的做法: 提前准备好符号表,把关键字 还有库函数添加到符号表中,
    /// 然后next函数只识别标识符, 并不区分 关键字 还是库函数,或者普通变量
    /// 文档上看到说go语言解析可以不用符号表,不知道是什么意思.
    /// 只有25个关键字, 不知是不是和词法解析有关系. 关键字和预定义标识符等内在关系上面
    fn try_next_token(
        iter: &mut std::str::Chars,
        ts: &mut TokenState,
    ) -> Option<Result<Token, LexError>> {
        // 不可以使用for in, into iter 会move走迭代器,就不能手动控制了
        loop {
            // 6.4 Lexical elements
            // token:
            //      keyword
            //      identifier
            //      constant: int, float, enum, char
            //      string-literal
            //      punctuator

            match iter.next() {
                None => {
                    return None;
                }
                Some(c) => match c {
                    // 处理换行和空白
                    '\r' => {
                        iter.peeking_take_while(|&x| x == '\n').next();
                        ts.line += 1;
                    }
                    '\n' => {
                        ts.line += 1;
                    }
                    ' ' | '\t' => {} // skip

                    // 处理广义上的标识符, 应该包括关键字和enum 常量
                    _ if is_id_initial_char(&c) => {
                        let mut ids = String::from(c);
                        while let Some(idc) = iter.peeking_take_while(is_id_char).next() {
                            ids.push(idc);
                        }
                        ts.token_count += 1;
                        return Some(Ok(Token::Id(ids)));
                    }
                    // const处理, 应该包含int, float, char
                    ch if is_digit(&c) => {
                        let mut iv = (ch as u8 - '0' as u8) as i64;
                        while let Some(nch) = iter.peeking_take_while(is_digit).next() {
                            iv = iv * 10 + ((nch as u8) - ('0' as u8)) as i64;
                        }
                        ts.token_count += 1;
                        return Some(Ok(Token::IntegerConst(iv)));
                    }
                    // string literal
                    '"' => {
                        return Self::try_string_literal(iter);
                    }

                    // punctuators
                    // report error for unknown & unexpected input
                    _ => return Some(Err(LexError::InvalidChar(c))),
                },
            };
        }
    }

    fn try_string_literal(iter: &mut std::str::Chars) -> Option<Result<Token, LexError>> {
	// 找到匹配的 " 之前, 匹配任何内容,并放入字符串常量; 需要处理转义,和 输入提前结束的异常


        None
    }

    /// 对输入字符串进行词法解析,得到一组token list,或者错误信息
    pub fn parse(input: &str) -> Result<Vec<Token>, LexError> {
        let mut vt = vec![];
        let mut ts = TokenState {
            line: 1,
            token_count: 0,
        };
        let mut chars_iter = input.chars();
        while let Some(r) = Self::try_next_token(&mut chars_iter, &mut ts) {
            match r {
                Ok(tk) => vt.push(tk),
                Err(e) => return Err(e),
            }
        }
        println!("ts: {:#?}", ts);

        Ok(vt)
    }
}

/// 词法分析状态
#[derive(Debug)]
struct TokenState {
    /// 当前行号
    line: isize,
    token_count: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lex_1() {
        assert_eq!(TokenApi::parse("123"), Ok(vec![Token::IntegerConst(123)]));
        assert_eq!(
            TokenApi::parse("1 23"),
            Ok(vec![Token::IntegerConst(1), Token::IntegerConst(23)])
        );
        // assert_eq!(
        //     TokenApi::parse("1x23"),
        //     vec![Token::Num(1), Token::Id("x23".to_string())]
        // );
    }

    #[test]
    fn run_lex_2() {
        // assert_eq!(TokenApi::parse("if"), vec![Token::Id("if".to_string())]);
        // assert_eq!(TokenApi::parse("ix"), vec![Token::Id("ix".to_string())]);
        // assert_eq!(
        //     TokenApi::parse("if ix"),
        //     vec![Token::Id("if".to_string()), Token::Id("ix".to_string())]
        // );
        // assert_eq!(
        //     TokenApi::parse("else 123"),
        //     vec![Token::Id("else".to_string()), Token::Num(123)]
        // );
        // assert_eq!(
        //     TokenApi::parse("if_123"),
        //     vec![Token::Id("if_123".to_string())]
        // );
    }

    #[test]
    fn test_put_back() {
        let mut c = itertools::put_back("hello".chars());
        c.put_back('X');
        c.put_back('Y'); // 会覆盖上一次,因为内部只有一个空间
        for v in c {
            println!("{}", v);
        }
        let mut pn = itertools::put_back_n("hello".chars());
        pn.put_back('Z');
        pn.put_back('Y');
        pn.put_back('X');
        for v in pn {
            println!("{}", v);
        }
    }
}
