pub enum TokenType {
    Number(f32),
    Operator(&'static str),
}

pub fn join(tokens :&Vec<TokenType>, separator :&str) -> String {
    tokens.iter().map(|t| {
        match t {
            &TokenType::Number(n) => n.to_string(),
            &TokenType::Operator(s) => s.to_string()
        }
    })
    .fold(String::new(), |acc, s| {
        let sep = if acc == "" { "" } else { separator };
        acc + sep + &s
    })
}

