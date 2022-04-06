use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    CREATE,
    INSERT,
    INTO,
    LPARENT,
    RPARENT,
    COMMA,
    DATABASE,
    IDENTIFIER(String),
}

pub fn get_keyword_token<'a>(value: String) -> Token {
    match &value.to_lowercase()[..] {
        "create" => Token::CREATE,
        "insert" => Token::INSERT,
        "into" => Token::INTO,
        "database" => Token::DATABASE,
        _ => Token::IDENTIFIER(value.to_string()),
    }
}

pub fn next_token(chars: &mut Chars) -> Option<Token> {
    let ch = chars.next()?;

    let tok: Token;
    match ch {
        '(' => tok = Token::LPARENT,
        ')' => tok = Token::RPARENT,
        ',' => tok = Token::COMMA,
        _ => {
            let mut identifier: Vec<char> = vec![];
            if ch.is_alphanumeric() || ch.eq(&'_') {
                identifier.push(ch);
            }

            loop {
                if let Some(ch) = chars.next() {
                    if ch.is_alphanumeric() || ch.eq(&'_') {
                        identifier.push(ch);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let value = identifier.iter().collect::<String>();
            let token = get_keyword_token(value);
            return Some(token);
        }
    };

    Some(tok)
}

pub fn parse(query: &str) -> Option<Vec<Token>> {
    let mut chars = query.chars();
    let mut tokens: Vec<Token> = vec![];

    loop {
        let token = next_token(&mut chars);
        if token.is_none() {
            break;
        }
        tokens.push(token?);
    }

    Some(tokens)
}

#[cfg(test)]
mod query_test {
    use crate::query::parse;
    use crate::query::Token;
    use crate::query::Token::*;

    #[test]
    pub fn create_db_query() {
        let expected_tokens = vec![CREATE, DATABASE, IDENTIFIER("users".to_string())];
        let actual_tokens = parse("create database users").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![CREATE, DATABASE, IDENTIFIER("users1".to_string())];
        let actual_tokens = parse("create database users1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![CREATE, DATABASE, IDENTIFIER("users_1".to_string())];
        let actual_tokens = parse("create database users_1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    pub fn create_insert_query() {
        let expected_tokens = vec![INSERT, INTO, IDENTIFIER("role".to_string())];
        let actual_tokens = parse("insert into role").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![INSERT, INTO, IDENTIFIER("role1".to_string())];
        let actual_tokens = parse("insert into role1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![INSERT, INTO, IDENTIFIER("role_1".to_string())];
        let actual_tokens = parse("insert into role_1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![
            INSERT,
            INTO,
            IDENTIFIER("role_1".to_string()),
            Token::LPARENT,
            Token::IDENTIFIER("column1".to_string()),
            Token::IDENTIFIER("column2".to_string()),
            // Token::RPARENT, // fix this
        ];
        let actual_tokens = parse("insert into role_1 (column1, column2)").unwrap();
        assert_eq!(expected_tokens, actual_tokens);
    }
}
