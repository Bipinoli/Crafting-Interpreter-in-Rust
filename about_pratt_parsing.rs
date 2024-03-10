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

struct Lexer {
    tokens: Vec<Token>,
    cursor: usize,
}
impl Lexer {
    fn new(tokens: Vec<Token>) -> Self {
        Lexer {
            tokens: tokens, 
            cursor: 0
        }
    }
    fn next(&mut self) -> &Token {
        let retval = &self.tokens[self.cursor];
        self.cursor += 1;
        retval
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }
}



// Binding power as per precedence rule (div/mult > add/sub)
// for 2 + 3 + 4 we want (2 + 3) + 4
// so the operator should have slighly higher binding force towards the right
struct BindingPower {
    left: f32,
    right: f32,
}
fn get_binding_power(operator: &Token) -> BindingPower {
    match operator {
        Token::Op(op_char) => {
            match op_char {
                '-' => BindingPower {left: 1.0, right: 1.1},
                '+' => BindingPower {left: 2.0, right: 2.1},
                '*' => BindingPower {left: 3.0, right: 3.1},
                '/' => BindingPower {left: 4.0, right: 4.1},
                _ => panic!("unknown operator"),
            }
        }
        _ => panic!("only operators have binding power")
    }
}

enum ParseTree<'a> {
    Leaf(Token),
    Branch { operator: Token, left: &'a ParseTree<'a>, right: &'a ParseTree<'a> },
}

fn pratt_parse(lexer: &Lexer) {
    todo!("parse() doesn't consume everythign -- make sure to run that until END");
}
 
fn parse(existing_left: Option<ParseTree<'a>>, lexer: &mut Lexer) -> ParseTree<'a> {
    let token = lexer.next();
    if let Token::End = token {
        return match existing_left {
            Some(v) => v,
            None => ParseTree::Leaf(Token::End),
        };
    }
    if let Token::Op(_) = token {
        panic!("first token can't be an operator");
    }
    let left_operator = match existing_left {
        Some(left) => (left, token),
        None => (ParseTree::Leaf(token), lexer.next())
    };
    let left = left_operator.0;
    let operator = left_operator.1;
    let mut right: ParseTree = ParseTree::Leaf(lexer.next());
    loop {
        let next_operator = lexer.peek();
        if get_binding_power(*operator).right >= get_binding_power(next_operator).left {
            return ParseTree::Branch { operator: operator, left: Box::new(left), right: Box::new(right)};
        }
        right = parse(Some(right), lexer);
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
