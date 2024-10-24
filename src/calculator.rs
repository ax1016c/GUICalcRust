use std::f64::consts::{E, PI};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    Number(f64),
    Op(Operator),
    Bracket(char),
    Function(Function),
    Constant(Constant),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Operator {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Pow,      // ^
    Mod,      // %
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sqrt,
    Cbrt,
    Log,
    Log10,
    Abs,
    Floor,
    Ceil,
    Round,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Constant {
    Pi,
    E,
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
    InvalidNumber(String),
    DivisionByZero,
    InvalidOperation(String),
    UnknownFunction(String),
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Sub => 1,
            Operator::Mul | Operator::Div | Operator::Mod => 2,
            Operator::Pow => 3,
        }
    }
}

pub struct Calculator {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref().to_lowercase();
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();
        let mut parens = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' | '.' => {
                    let mut number = String::from(c);
                    while let Some(&next) = chars.peek() {
                        if next.is_digit(10) || next == '.' || next == 'e' {
                            number.push(chars.next().unwrap());
                            if next == 'e' {
                                if let Some(&sign) = chars.peek() {
                                    if sign == '+' || sign == '-' {
                                        number.push(chars.next().unwrap());
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    match number.parse::<f64>() {
                        Ok(n) => tokens.push(Token::Number(n)),
                        Err(_) => return Err(Error::InvalidNumber(number)),
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    } else {
                        return Err(Error::MismatchedParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => {
                    // Handle negative numbers
                    if tokens.is_empty() || matches!(tokens.last(), 
                        Some(Token::Op(_)) | Some(Token::Bracket('('))) {
                        tokens.push(Token::Number(-1.0));
                        tokens.push(Token::Op(Operator::Mul));
                    } else {
                        tokens.push(Token::Op(Operator::Sub));
                    }
                },
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '^' => tokens.push(Token::Op(Operator::Pow)),
                '%' => tokens.push(Token::Op(Operator::Mod)),
                'p' => {
                    if expr[expr.find(c).unwrap()..].starts_with("pi") {
                        tokens.push(Token::Constant(Constant::Pi));
                        chars.next(); // skip 'i'
                    } else {
                        return Err(Error::BadToken(c));
                    }
                },
                'e' => {
                    if chars.peek().is_none() || !chars.peek().unwrap().is_alphabetic() {
                        tokens.push(Token::Constant(Constant::E));
                    }
                },
                's' => {
                    if expr[expr.find(c).unwrap()..].starts_with("sin") {
                        tokens.push(Token::Function(Function::Sin));
                        chars.next(); chars.next(); // skip "in"
                    } else if expr[expr.find(c).unwrap()..].starts_with("sqrt") {
                        tokens.push(Token::Function(Function::Sqrt));
                        chars.next(); chars.next(); chars.next(); // skip "qrt"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                'c' => {
                    if expr[expr.find(c).unwrap()..].starts_with("cos") {
                        tokens.push(Token::Function(Function::Cos));
                        chars.next(); chars.next(); // skip "os"
                    } else if expr[expr.find(c).unwrap()..].starts_with("cbrt") {
                        tokens.push(Token::Function(Function::Cbrt));
                        chars.next(); chars.next(); chars.next(); // skip "brt"
                    } else if expr[expr.find(c).unwrap()..].starts_with("ceil") {
                        tokens.push(Token::Function(Function::Ceil));
                        chars.next(); chars.next(); chars.next(); // skip "eil"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                't' => {
                    if expr[expr.find(c).unwrap()..].starts_with("tan") {
                        tokens.push(Token::Function(Function::Tan));
                        chars.next(); chars.next(); // skip "an"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                'l' => {
                    if expr[expr.find(c).unwrap()..].starts_with("log10") {
                        tokens.push(Token::Function(Function::Log10));
                        chars.next(); chars.next(); chars.next(); chars.next(); // skip "og10"
                    } else if expr[expr.find(c).unwrap()..].starts_with("log") {
                        tokens.push(Token::Function(Function::Log));
                        chars.next(); chars.next(); // skip "og"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                'a' => {
                    if expr[expr.find(c).unwrap()..].starts_with("abs") {
                        tokens.push(Token::Function(Function::Abs));
                        chars.next(); chars.next(); // skip "bs"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                'f' => {
                    if expr[expr.find(c).unwrap()..].starts_with("floor") {
                        tokens.push(Token::Function(Function::Floor));
                        chars.next(); chars.next(); chars.next(); chars.next(); // skip "loor"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                'r' => {
                    if expr[expr.find(c).unwrap()..].starts_with("round") {
                        tokens.push(Token::Function(Function::Round));
                        chars.next(); chars.next(); chars.next(); chars.next(); // skip "ound"
                    } else {
                        return Err(Error::UnknownFunction(c.to_string()));
                    }
                },
                ' ' | '\n' => {},
                _ => return Err(Error::BadToken(c))
            }
        }
        
        if !parens.is_empty() {
            return Err(Error::MismatchedParens);
        }
        
        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();
        
        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        
        while let Some(token) = tokens.pop() {
            match &token {
                Token::Number(_) => queue.push(token),
                Token::Constant(_) => queue.push(token),
                Token::Op(op) => {
                    while let Some(Token::Op(top_op)) = stack.last() {
                        if op.precedence() <= top_op.precedence() {
                            queue.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    stack.push(token);
                },
                Token::Function(_) => stack.push(token),
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while let Some(top) = stack.last() {
                        if matches!(top, Token::Bracket('(')) {
                            stack.pop();
                            if let Some(Token::Function(_)) = stack.last() {
                                queue.push(stack.pop().unwrap());
                            }
                            break;
                        }
                        queue.push(stack.pop().unwrap());
                    }
                },
                _ => {}
            }
        }
        
        while let Some(token) = stack.pop() {
            if !matches!(token, Token::Bracket('(')) {
                queue.push(token);
            }
        }
        
        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Result<f64, Error> {
        tokens.reverse();
        
        let mut stack: Vec<f64> = Vec::new();
        
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num),
                Token::Constant(constant) => {
                    match constant {
                        Constant::Pi => stack.push(PI),
                        Constant::E => stack.push(E),
                    }
                },
                Token::Op(op) => {
                    if stack.len() < 2 {
                        return Err(Error::InvalidOperation("No hay suficientes operandos".to_string()));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    
                    let result = match op {
                        Operator::Add => left + right,
                        Operator::Sub => left - right,
                        Operator::Mul => left * right,
                        Operator::Div => {
                            if right == 0.0 {
                                return Err(Error::DivisionByZero);
                            }
                            left / right
                        },
                        Operator::Pow => left.powf(right),
                        Operator::Mod => {
                            if right == 0.0 {
                                return Err(Error::DivisionByZero);
                            }
                            left % right
                        },
                    };
                    stack.push(result);
                },
                Token::Function(func) => {
                    if stack.is_empty() {
                        return Err(Error::InvalidOperation("No hay suficientes operandos para la función".to_string()));
                    }
                    let val = stack.pop().unwrap();
                    
                    let result = match func {
                        Function::Sin => val.sin(),
                        Function::Cos => val.cos(),
                        Function::Tan => val.tan(),
                        Function::Sqrt => {
                            if val < 0.0 {
                                return Err(Error::InvalidOperation("No se puede sacar raíz cuadrada de un número negativo".to_string()));
                            }
                            val.sqrt()
                        },
                        Function::Cbrt => val.cbrt(),
                        Function::Log => {
                            if val <= 0.0 {
                                return Err(Error::InvalidOperation("No se puede tomar el logaritmo de un número no positivo".to_string()));
                            }
                            val.ln()
                        },
                        Function::Log10 => {
                            if val <= 0.0 {
                                return Err(Error::InvalidOperation("No se puede tomar el logaritmo de un número no positivo".to_string()));
                            }
                            val.log10()
                        },
                        Function::Abs => val.abs(),
                        Function::Floor => val.floor(),
                        Function::Ceil => val.ceil(),
                        Function::Round => val.round(),
                    };
                    stack.push(result);
                },
                _ => {}
            }
        }
        
        if stack.len() != 1 {
            return Err(Error::InvalidOperation("Expresión inválida".to_string()));
        }
        
        Ok(stack.pop().unwrap())
    }
}