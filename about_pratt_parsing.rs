/*
 * Pratt Parsing:
 *
 * Main idea:
 *      Each operator has a attractive binding force. Operands are pulled towards the higher
 *      force (eg. binding power).
 *
 * Example:
 *      2 + 3 / 2
 *      / has higher pulling force than +
 *      so, 3 gets pulled to the / operator
 *
 *  For case:
 *      2 + 3 + 2
 *      if we want (2 + 3) + 2
 *      then we should provide a slighly higher attractive force towards the right side of the
 *      operator, so that the first + has more pulling force to pull 3 compared to second +
 *
 */

#[derive(Debug)]
enum Token {
    Num(f64),
    Op(char),
    End
}

// Binding power as per precedence rule (div/mult > add/sub)
// for 2 + 3 + 4 we want (2 + 3) + 4
// so the operator should have slighly higher binding force towards the right
fn get_binding_power(operator: char) -> (f32, f32) {
    match operator {
        '-' => (1.0, 1.1),
        '+' => (2.0, 2.1),
        '*' => (3.0, 3.1),
        '/' => (4.0, 4.1),
        _ => panic!("unknown operator"),
    }
}

enum ParseTree {
    Leaf(f64),
    Branch(char, Vec<ParseTree>),
}

fn parse(cursor: usize, tokens: &Vec<Token>, binding: f32) -> ParseTree {
    let left = get_operand(&mut cursor, tokens);
    let operator = get_operator(&mut cursor, tokens);
    let right = get_operand(&mut cursor, tokens);
    let next_operator = next_operator(cursor, tokens);

}


fn get_operand(cursor: &mut usize, tokens: &Vec<Token>) -> f64 {
    let retval = match tokens[*cursor] {
        Token::Op(c) => panic!("expected number but found operator '{}'", c),
        Token::Num(n) => n
    };
    *cursor += 1;
    retval
}

fn get_operator(cursor: &mut usize, tokens: &Vec<Token>) -> char {
    let retval = match tokens[*cursor] {
        Token::Op(c) => c,
        Token::Num(n) => panic!("expected operator but found number {}", n)
    };
    *cursor += 1;
    retval
}

fn next_operator(cursor: usize, tokens: &Vec<Token) -> char {
    match tokens[*cursor] = {
        Token::Op(c) => c,
        Token::Num(n) => panic!("epxected operator but found a number {}", n),
    }
}

fn main() {
    // case: 2 - 6 / 2 + 2 * 4
    // expected: 2 - ((6 / 2) + (2 * 4))
    let tokens = vec![
        Token::Num(2.0),
        Token::Op('-'),
        Token::Num(6.0),
        Token::Op('/'),
        Token::Num(2.0),
        Token::Op('+'),
        Token::Num(2.0),
        Token::Op('*'),
        Token::Num(4.0),
    ];
    dbg!(&tokens);
}
