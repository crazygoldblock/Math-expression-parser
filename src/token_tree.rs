use std::mem::replace;

use crate::{token_parse::{operator_to_string, Operator, Token}, DEBUG};

pub fn evaluate_exp(tokens: &[Token]) -> f64 {

    let mut parser = Parser::new(tokens.to_vec());
    create_eval_tree_recursive(&mut parser)
}
fn create_eval_tree_recursive(parser: &mut Parser) -> f64 {

    let mut tree = TokenTree::new();
    let mut operator = Operator::Plus;

    while parser.has_next() {
        match parser.consume() {
            Token::Number(n) => {
                tree.add_next(operator, *n);
            },
            Token::Operator(o) => {
                operator = *o;
            },
            Token::Bracket(b) => {
                if *b { tree.add_next(operator, create_eval_tree_recursive(parser)); }
                else {
                    if DEBUG { tree.print(); }
                    return tree.evaluate_tree(); 
                }
            },
        }
    }
    if DEBUG { tree.print(); }
    return tree.evaluate_tree();
}

struct Parser {
    tokens: Box<[Token]>,
    index: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens.into(), index: 0 }
    }
    fn consume(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index - 1]
    }
    fn has_next(&self) -> bool {
        self.tokens.len() > self.index
    }
}

struct TokenTree {
    tokens: Vec<Node>,
    base: usize,
    last: usize,
} 
impl TokenTree {
    fn new() -> Self {
        TokenTree { tokens: Vec::new(), base: 0, last: 0 }
    }
    fn add_last(&mut self, o: Operator, n: f64) {
        let node = Node::Operator(o, self.tokens.len(), self.tokens.len() + 1);

        let last = replace(&mut self.tokens[self.last], node);

        self.tokens.push(last);
        self.tokens.push(Node::Number(n));

        self.last = self.tokens.len() - 1;
    }
    fn add_base(&mut self, o: Operator, n: f64) {

        let node = Node::Operator(o, self.base, self.tokens.len());

        self.tokens.push( Node::Number(n) );
        self.tokens.push(node);

        self.base = self.tokens.len() - 1;
        self.last = self.tokens.len() - 2;
    }
    fn add_next(&mut self, o: Operator, n: f64) {

        if self.tokens.len() == 0 {
            self.tokens.push(Node::Number(n));
            return;
        }

        match o {
            Operator::Plus | Operator::Minus => self.add_base(o, n),
            Operator::Mul | Operator::Div => self.add_last(o, n),
        }
    }
    fn evaluate_tree(&mut self) -> f64 {

        if self.tokens.len() < 10_000 {
            self.evaluate_node_stack(&self.tokens[self.base])
        }   
        else {
            self.evaluate_node_heap()
        }
    }
    fn evaluate_node_stack(&self, node: &Node) -> f64 {
        match node {
            Node::Number(n) => *n,
            Node::Operator(o, l, r) => {
                let left = self.evaluate_node_stack(&self.tokens[*l]);
                let right = self.evaluate_node_stack(&self.tokens[*r]);
                match o {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => left / right,
                }
            },
        }
    }
    fn evaluate_node_heap(&mut self) -> f64 {
        let mut nodes = Vec::with_capacity(self.tokens.len() / 2);
        
        nodes.push(self.base);

        while nodes.len() > 0 {
            let index = nodes.pop().unwrap();
            if let Node::Operator(o, l, r) = self.tokens[index] {
                match (self.tokens[l].clone(), self.tokens[r].clone()) {
                    (Node::Number(n1), Node::Number(n2)) => {
                        let res = match o {
                            Operator::Plus => n1 + n2,
                            Operator::Minus => n1 - n2,
                            Operator::Mul => n1 * n2,
                            Operator::Div => n1 / n2,
                        };
                        self.tokens[index] = Node::Number(res);
                        continue;
                    },
                    (Node::Number(_), Node::Operator(_, _, _)) => nodes.push(r),
                    (Node::Operator(_, _, _), Node::Number(_)) => nodes.push(l),
                    (Node::Operator(_, _, _), Node::Operator(_, _, _)) => { nodes.push(l); nodes.push(r); },
                }
                nodes.insert(nodes.len() - 1, index);
            }
        }

        if let Node::Number(n) = self.tokens[self.base] {
            return n;
        }
        unreachable!();
    }
    pub fn print(&self) {
        println!("TREE - base: {}, last:  {}", self.base, self.last);
        for (i, n) in self.tokens.iter().enumerate() {
            print!("{} ", i);
            match n {
                Node::Number(n) => println!("N: {}", n),
                Node::Operator(o, l, r) => println!("Op: {}, {}, {}", operator_to_string(o), l, r),
            }
        }
    }
}
#[derive(Clone)]
enum Node {
    Number(f64),
    Operator(Operator, usize, usize),
}