use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    NumberNode(f32),
    AddNode(Box<Node>, Box<Node>),
    SubtractNode(Box<Node>, Box<Node>),
    MultiplyNode(Box<Node>, Box<Node>),
    DivideNode(Box<Node>, Box<Node>),
    PlusNode(Box<Node>),
    MinusNode(Box<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Node::*;
        match self {
            NumberNode(x) => write!(f, "{}", x),
            AddNode(a, b) => write!(f, "({}+{})", a, b),
            SubtractNode(a, b) => write!(f, "({}-{})", a, b),
            MultiplyNode(a, b) => write!(f, "({}*{})", a, b),
            DivideNode(a, b) => write!(f, "({}/{})", a, b),
            PlusNode(node) => write!(f, "(+{})", node),
            MinusNode(node) => write!(f, "(-{})", node),
        }
    }
}
