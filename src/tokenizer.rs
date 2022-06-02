#[derive(Debug, Clone, Copy)]
enum TokenizerState {
    // accept any token that is an expression from the left: var, num, (, negpos
    LExpr,
    // accept any token that needs an expression on the left: fact, binop, ), comma
    AfterRExpr,
}

pub fn tokenize<S: AsRef<str>>(input: S) -> Result<Vec<Token>, ParseError> {
    use self::TokenizerState::*;
    use nom::Err;
    use nom::IResult::*;
    let mut state = LExpr;
    // number of function arguments left
    let mut paren_stack = vec![];

    let mut res = vec![];

    let input = input.as_ref().as_bytes();
    let mut s = input;

    while !s.is_empty() {
        let r = match (state, paren_stack.last()) {
            (LExpr, _) => lexpr(s),
            (AfterRExpr, None) => after_rexpr_no_paren(s),
            (AfterRExpr, Some(&ParenState::Subexpr)) => after_rexpr(s),
            (AfterRExpr, Some(&ParenState::Func)) => after_rexpr_comma(s),
        };

        match r {
            Done(rest, t) => {
                match t {
                    Token::LParen => {
                        paren_stack.push(ParenState::Subexpr);
                    }
                    Token::Func(..) => {
                        paren_stack.push(ParenState::Func);
                    }
                    Token::RParen => {
                        paren_stack.pop().expect("The paren_stack is empty!");
                    }
                    Token::Var(_) | Token::Number(_) => {
                        state = AfterRExpr;
                    }
                    Token::Binary(_) | Token::Comma => {
                        state = LExpr;
                    }
                    _ => {}
                }
                res.push(t);
                s = rest;
            }
            Error(Err::Position(_, p)) => {
                let (i, _) = slice_to_offsets(input, p);
                return Err(ParseError::UnexpectedToken(i));
            }
            _ => {
                panic!(
                    "Unexpected parse result when parsing `{}` at `{}`: {:?}",
                    String::from_utf8_lossy(input),
                    String::from_utf8_lossy(s),
                    r
                );
            }
        }
    }

    match state {
        LExpr => Err(ParseError::MissingArgument),
        _ if !paren_stack.is_empty() => Err(ParseError::MissingRParen(paren_stack.len() as i32)),
        _ => Ok(res),
    }
}
