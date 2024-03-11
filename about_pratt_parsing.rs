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

#[derive(Debug, Clone)]
enum Token {
    Num(f64),
    Op(char),
    End,
}

struct Lexer {
    tokens: Vec<Token>,
    cursor: usize,
}
impl Lexer {
    fn new(tokens: Vec<Token>) -> Self {
        Lexer {
            tokens: tokens,
            cursor: 0,
        }
    }
    fn next(&mut self) -> Token {
        let retval = self.tokens[self.cursor].clone();
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
        Token::Op(op_char) => match op_char {
            '-' => BindingPower {
                left: 1.0,
                right: 1.1,
            },
            '+' => BindingPower {
                left: 2.0,
                right: 2.1,
            },
            '*' => BindingPower {
                left: 3.0,
                right: 3.1,
            },
            '/' => BindingPower {
                left: 4.0,
                right: 4.1,
            },
            _ => panic!("unknown operator"),
        },
        _ => panic!("only operators have binding power"),
    }
}

#[derive(Debug)]
enum ParseTree {
    Leaf(Token),
    Branch {
        operator: Token,
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
}

fn pratt_parse(lexer: &mut Lexer) -> ParseTree {
    let mut left: Option<ParseTree> = None;
    loop {
        let tree = parse(left, lexer);
        if let Token::End = lexer.peek() {
            return tree;
        }
        left = Some(tree);
    }
}

fn parse(left: Option<ParseTree>, lexer: &mut Lexer) -> ParseTree {
    if let Token::End = lexer.peek() {
        return match left {
            Some(v) => v,
            None => ParseTree::Leaf(Token::End),
        };
    }
    let left_operator = match left {
        Some(left) => (left, lexer.next()),
        None => (ParseTree::Leaf(lexer.next()), lexer.next()),
    };
    let left = left_operator.0;
    let operator = left_operator.1;
    let mut right: ParseTree = ParseTree::Leaf(lexer.next());
    loop {
        let next_operator = lexer.peek();
        if let Token::End = next_operator {
            return ParseTree::Branch {
                operator: operator.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        if get_binding_power(&operator).right >= get_binding_power(next_operator).left {
            return ParseTree::Branch {
                operator: operator.clone(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        right = parse(Some(right), lexer);
    }
}

fn pretty_print(parse_tree: &ParseTree) -> String {
    match parse_tree {
        ParseTree::Leaf(v) => match v {
            Token::End => String::from("END"),
            Token::Num(n) => format!("{}", n),
            Token::Op(c) => format!("{}", c),
        },
        ParseTree::Branch {
            operator,
            left,
            right,
        } => {
            format!(
                "({} {} {})",
                pretty_print(&**left),
                {
                    match operator {
                        Token::Op(c) => format!("{}", c),
                        _ => panic!("invalid operator"),
                    }
                },
                pretty_print(&**right)
            )
        }
    }
}

fn main() {
    // case: 2 - 6 / 2 + 2 * 4
    let expected = "(2 - ((6 / 2) + (2 * 4)))".to_owned();
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
        Token::End,
    ];

    let mut lexer = Lexer::new(tokens);

    let parse_result = pratt_parse(&mut lexer);
    let pretty = pretty_print(&parse_result);

    println!("{}", pretty);
    assert_eq!(pretty, expected);

    // case: 1 + 4 / 2 - 3 * 2 - 1
    let expected = "(((1 + (4 / 2)) - (3 * 2)) - 1)".to_owned();
    let tokens = vec![
        Token::Num(1.0),
        Token::Op('+'),
        Token::Num(4.0),
        Token::Op('/'),
        Token::Num(2.0),
        Token::Op('-'),
        Token::Num(3.0),
        Token::Op('*'),
        Token::Num(2.0),
        Token::Op('-'),
        Token::Num(1.0),
        Token::End,
    ];

    let mut lexer = Lexer::new(tokens);

    let parse_result = pratt_parse(&mut lexer);
    let pretty = pretty_print(&parse_result);

    println!("{}", pretty);
    assert_eq!(pretty, expected);
}
