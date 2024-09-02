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
        let node = Node::Operator( NodeOp { operator: o, left: self.tokens.len(), right: self.tokens.len() + 1 } );

        let last = replace(&mut self.tokens[self.last], node);

        self.tokens.push(last);
        self.tokens.push(Node::Number(n));

        self.last = self.tokens.len() - 1;
    }
    fn add_base(&mut self, o: Operator, n: f64) {

        let node = Node::Operator( NodeOp { operator: o, left: self.base, right: self.tokens.len() } );
        

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
            Node::Operator(op) => {
                let left = self.evaluate_node_stack(&self.tokens[op.left]);
                let right = self.evaluate_node_stack(&self.tokens[op.right]);
                match op.operator {
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
        
        match self.tokens[self.base].clone() {
            Node::Number(n) => return n,
            Node::Operator(op) => nodes.push((self.base, op)),
        }

        loop {
            let (index, op) = nodes.last().unwrap().clone();
            match (self.tokens[op.left].clone(), self.tokens[op.right].clone()) {
                (Node::Number(n1), Node::Number(n2)) => {
                    let res = match op.operator {
                        Operator::Plus => n1 + n2,
                        Operator::Minus => n1 - n2,
                        Operator::Mul => n1 * n2,
                        Operator::Div => n1 / n2,
                    };

                    if nodes.len() == 0 {
                        return res;
                    }

                    self.tokens[index] = Node::Number(res);
                    nodes.pop();
                },
                (Node::Number(_), Node::Operator(op2)) => {
                    nodes.push((op.right, op2));
                }
                    
                (Node::Operator(op1), Node::Number(_)) => {
                    nodes.push((op.left, op1));
                }
                (Node::Operator(op1), Node::Operator(op2)) => { 
                    nodes.push((op.left, op1)); 
                    nodes.push((op.right, op2)); 
                },
            }
        }
    }
    pub fn print(&self) {
        println!("TREE - base: {}, last:  {}", self.base, self.last);
        for (i, n) in self.tokens.iter().enumerate() {
            print!("{} ", i);
            match n {
                Node::Number(n) => println!("N: {}", n),
                Node::Operator(op) => println!("Op: {}, {}, {}", operator_to_string(&op.operator), op.left, op.right),
            }
        }
    }
}
#[derive(Clone)]
enum Node {
    Number(f64),
    Operator(NodeOp),
}
#[derive(Clone)]
struct NodeOp {
    operator: Operator,
    left: usize,
    right: usize,
}