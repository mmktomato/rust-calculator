use std::env;

mod tokenutil;

fn main() {
    let args = env::args();
    let s = args.skip(1).fold(
        String::new(),
        |acc, a| { acc + &a });

    // tokenize.
    let tokens = tokenize(&(s.trim()));

    // validate.
    validate(&tokens);
    println!("expression is '{}' .", tokenutil::join(&tokens, " "));

    // calculate.
    let res = calculate(&tokens);

    println!("result is {} .", res);
}

// tokenize expr to number and operator.
fn tokenize(expr :&str) -> Vec<tokenutil::TokenType> {
    // if expr is a number, return as it is.
    let num :Result<f32, _> = expr.parse();
    if num.is_ok() {
        return vec![tokenutil::TokenType::Number(num.unwrap())];
    }

    // if expr is [+,-,*,/], return as it is.
    let mut tokens = match expr {
        "+" => vec![tokenutil::TokenType::Operator("+")],
        "-" => vec![tokenutil::TokenType::Operator("-")],
        "*" => vec![tokenutil::TokenType::Operator("*")],
        "/" => vec![tokenutil::TokenType::Operator("/")],
        _ => vec![]
    };
    if tokens.len() == 1 {
        return tokens;
    }

    let fn_split = |c, with_c| {
        let tmp = expr.split(c);
        let mut res :Vec<tokenutil::TokenType> = Vec::new();

        for token in tmp {
            if with_c && res.len() != 0 {
                res.push(tokenutil::TokenType::Operator(c));
            }
            for token_ in tokenize(token) {
                res.push(token_);
            }
        }

        res
    };

    if expr.contains(" ") {
        tokens = fn_split(" ", false);
    }
    else if expr.contains("+") {
        tokens = fn_split("+", true);
    }
    else if expr.contains("-") {
        tokens = fn_split("-", true);
    }
    else if expr.contains("*") {
        tokens = fn_split("*", true);
    }
    else if expr.contains("/") {
        tokens = fn_split("/", true);
    }

    tokens
}

// validate expression.
fn validate(tokens :&Vec<tokenutil::TokenType>) {
    // if tokens' length is 0, invalid.
    if tokens.is_empty() {
        panic!("expression is empty.");
    }

    // if expression starts with operator, invalid.
    match tokens[0] {
        tokenutil::TokenType::Operator(o) => panic!(format!("expression can't start with operator {} .", o)),
        tokenutil::TokenType::Number(_) => ()
    }

    // if expression ends with operator, invalid.
    match tokens[tokens.len() - 1] {
        tokenutil::TokenType::Operator(o) => panic!(format!("expression can't end with operator {} .", o)),
        tokenutil::TokenType::Number(_) => ()
    }

    // if the same type operators are continuous, invalid.
    let mut prev_token = &tokens[0];
    for token in tokens.iter().skip(1) {
        match (prev_token, token) {
            (&tokenutil::TokenType::Number(_), &tokenutil::TokenType::Operator(_)) => (),
            (&tokenutil::TokenType::Operator(_), &tokenutil::TokenType::Number(_)) => (),
            _ => panic!("same token type can't be continuous.")
        }
        prev_token = token;
    }
}

// calculate!
fn calculate(tokens: &Vec<tokenutil::TokenType>) -> f32 {
    if tokens.len() == 1 {
        return match tokens[0] {
            tokenutil::TokenType::Number(n) => n,
            _ => panic!("TODO: fix err message.")
        }
    }

    let fn_calc3 = |token_n1: &tokenutil::TokenType, token_operator: &tokenutil::TokenType, token_n2: &tokenutil::TokenType| {
        match (token_n1, token_operator, token_n2) {
            (&tokenutil::TokenType::Number(n1), &tokenutil::TokenType::Operator(o), &tokenutil::TokenType::Number(n2)) => {
                match o {
                    "+" => n1 + n2,
                    "-" => n1 - n2,
                    "*" => n1 * n2,
                    "/" => n1 / n2,
                    _ => panic!("TODO: fix err message.")
                }
            },
            _ => panic!("TODO: fix err message")
        }
    };

    if tokens.len() == 3 {
        return fn_calc3(&tokens[0], &tokens[1], &tokens[2])
    }
    else {
        for operator in ["/", "*", "+", "-"].iter() {
            let pos = tokens
                .iter()
                .position(|ref token|
                          if let &tokenutil::TokenType::Operator(o) = *token {
                              &o == operator
                          }
                          else {
                              false
                          });

            if pos.is_some() {
                let pos_operator = pos.unwrap();
                let pos_n1 = pos_operator - 1;
                let pos_n2 = pos_operator + 1;

                let new_num = fn_calc3(&tokens[pos_n1], &tokens[pos_operator], &tokens[pos_n2]);
                let mut new_tokens :Vec<tokenutil::TokenType> = Vec::new();
                for token in tokens.iter() {
                    // copy token.
                    // TODO: copy trait
                    let new_token = match *token {
                        tokenutil::TokenType::Number(n) => tokenutil::TokenType::Number(n),
                        tokenutil::TokenType::Operator(o) => tokenutil::TokenType::Operator(o)
                    };

                    new_tokens.push(new_token);
                }

                new_tokens.remove(pos_n1);
                new_tokens.remove(pos_operator);
                new_tokens.remove(pos_n1);
                new_tokens.insert(pos_n1, tokenutil::TokenType::Number(new_num));

                println!("{}", tokenutil::join(&new_tokens, " "));
                return calculate(&new_tokens)
            }
        }
    }
    panic!("TODO: fix err message")
}

