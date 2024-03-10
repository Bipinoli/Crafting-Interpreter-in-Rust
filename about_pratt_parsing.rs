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
    fn next(&mut self) -> Token {
        let retval = self.tokens[self.cursor];
        self.cursor += 1;
        retval
    }
    fn next_operator(&mut self) -> Token {
        let retval = match self.tokens[self.cursor] {
            Token::Op(c) => Token::Op(c),
            _ => panic!("not operator"),
        };
        self.cursor += 1;
        retval
    }
    fn next_operand(&mut self) -> Token {
        let retval = match self.tokens[self.cursor] {
            Token::Num(c) => Token::Num(c),
            _ => panic!("not operand"),
        };
        self.cursor += 1;
        retval
    }
    fn peek_next_operator(&self) -> Token {
        match self.tokens[self.cursor] {
            Token::Op(c) => Token::Op(c),
            _ => panic!("not operator")
        }
    }
}



// Binding power as per precedence rule (div/mult > add/sub)
// for 2 + 3 + 4 we want (2 + 3) + 4
// so the operator should have slighly higher binding force towards the right
struct BindingPower {
    left: f32,
    right: f32,
}
fn get_binding_power(operator: Token) -> BindingPower {
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

enum ParseTree {
    Leaf(Token),
    Branch { operator: Token, left: Box<ParseTree>, right: Box<ParseTree> },
}

fn pratt_parse(lexer: &Lexer) {
}
 
fn parse(left: Option<ParseTree>, lexer: &mut Lexer) -> ParseTree {
    let token = lexer.next();
    if let Token::End = token {
        return match left {
            Some(v) => v,
            None => ParseTree::Leaf(Token::End),
        };
    }
    if let Token::Op = token {
        panic!("first token can't be an operator");
    }
    let left = match left {
        Some(v) => v,
        None => token,
    };
    let operator = match left {
        Some(v) => token,
        None => lexer.next_operator();
    }
    let mut right: ParseTree = ParseTree::Leaf(lexer.next_operand());
    loop {
        let next_operator = lexer.peek_next_operator();
        if get_binding_power(operator).right >= get_binding_power(next_operator).left {
            return ParseTree::Branch(operator, left: ParseTree::Leaf(left), right);
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
