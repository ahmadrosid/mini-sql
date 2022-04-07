#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    CREATE,
    INSERT,
    DELETE,
    INTO,
    VALUES,
    LPARENT,
    RPARENT,
    EQ,
    COMMA,
    FROM,
    WHERE,
    DATABASE,
    IDENTIFIER(String),
    ILLEGAL,
}

pub fn get_keyword_token<'a>(value: String) -> Token {
    match &value.to_lowercase()[..] {
        "create" => Token::CREATE,
        "insert" => Token::INSERT,
        "delete" => Token::DELETE,
        "into" => Token::INTO,
        "values" => Token::VALUES,
        "from" => Token::FROM,
        "where" => Token::WHERE,
        "database" => Token::DATABASE,
        _ => Token::IDENTIFIER(value.to_string()),
    }
}

pub fn skip_whitespace(position: usize, chars: &Vec<char>) -> Option<usize> {
    let ch = chars.get(position)?;
    if !ch.is_whitespace() {
        return Some(position + 1);
    }

    let mut next_post = position + 1;
    loop {
        let ch = chars.get(next_post)?;
        if ch.is_whitespace() {
            next_post += 1;
        } else {
            break;
        }
    }

    Some(next_post)
}

pub fn next_token(position: usize, chars: &Vec<char>) -> Option<(usize, Token)> {
    let ch = chars.get(position)?;

    let tok: Token;
    match ch {
        '(' => tok = Token::LPARENT,
        ')' => tok = Token::RPARENT,
        ',' => tok = Token::COMMA,
        '=' => tok = Token::EQ,
        _ => {
            let mut identifier: Vec<&char> = vec![];
            if ch.is_alphanumeric() || ch.eq(&'_') {
                identifier.push(ch);
            }

            let mut next_pos = skip_whitespace(position, chars)?;
            while next_pos < chars.len() {
                let ch = chars.get(next_pos)?;
                if ch.is_alphanumeric() || ch.eq(&'_') {
                    identifier.push(ch);
                } else {
                    next_pos -= 1;
                    break;
                }
                next_pos += 1;
            }

            let value = identifier.into_iter().collect::<String>();
            if value.is_empty() {
                return Some((next_pos, Token::ILLEGAL));
            }

            let token = get_keyword_token(value);
            return Some((next_pos, token));
        }
    };

    Some((position, tok))
}

pub fn parse(query: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = query.chars().collect();
    let mut tokens: Vec<Token> = vec![];

    let mut position = 0;
    loop {
        let token = next_token(position, &chars);
        if token.is_none() {
            break;
        }

        let (pos, token) = token?;
        if token != Token::ILLEGAL {
            tokens.push(token);
        }
        position = pos + 1;
    }

    Some(tokens)
}

#[cfg(test)]
mod query_test {
    use crate::query::parse;
    use crate::query::Token::*;

    #[test]
    pub fn create_db_query() {
        let expected_tokens = vec![CREATE, DATABASE, IDENTIFIER("users".to_string())];
        let actual_tokens = parse("create database users").unwrap();
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

        let expected_tokens = vec![INSERT, INTO, IDENTIFIER("role_1".to_string())];
        let actual_tokens = parse("insert into role_1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![
            INSERT,
            INTO,
            IDENTIFIER("role_1".to_string()),
            LPARENT,
            IDENTIFIER("column1".to_string()),
            COMMA,
            IDENTIFIER("column2".to_string()),
            RPARENT,
        ];
        let actual_tokens = parse("insert into role_1 (column1, column2)").unwrap();
        assert_eq!(expected_tokens, actual_tokens);

        let expected_tokens = vec![
            INSERT,
            INTO,
            IDENTIFIER("role_1".to_string()),
            LPARENT,
            IDENTIFIER("column1".to_string()),
            COMMA,
            IDENTIFIER("column2".to_string()),
            RPARENT,
            VALUES,
            LPARENT,
            IDENTIFIER("val1".to_string()),
            COMMA,
            IDENTIFIER("val2".to_string()),
            RPARENT,
        ];
        let actual_tokens =
            parse("INSERT INTO role_1 (column1, column2) VALUES (val1, val2)").unwrap();
        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    pub fn test_delete_query() {
        let expected_tokens = vec![
            DELETE,
            FROM,
            IDENTIFIER("table_name".to_string()),
            WHERE,
            IDENTIFIER("id".to_string()),
            EQ,
            IDENTIFIER("1".to_string()),
        ];
        let actual_tokens = parse("DELETE FROM table_name WHERE id = 1").unwrap();
        assert_eq!(expected_tokens, actual_tokens);
    }
}
