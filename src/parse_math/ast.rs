#[derive(PartialEq, Debug)]
pub enum Node {
    Element(f64),
    Negative(Box<Node>),
    Sum(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
}

impl Node {
    pub fn eval(&self) -> f64 {
        match self {
            Self::Element(number) => *number,
            Self::Negative(node) => -node.eval(),
            Self::Sum(left, right) => left.eval() + right.eval(),
            Self::Subtract(left, right) => left.eval() - right.eval(),
            Self::Multiply(left, right) => left.eval() * right.eval(),
            Self::Divide(left, right) => left.eval() / right.eval(),
            Self::Power(left, right) => left.eval().powf(right.eval()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let node = Node::Element(3.);
        assert_eq!(node.eval(), 3.);
    }

    #[test]
    fn negative() {
        let node = Node::Negative(Box::new(Node::Element(3.)));
        assert_eq!(node.eval(), -3.);
    }

    #[test]
    fn multiply() {
        let node = Node::Multiply(Box::new(Node::Element(3.)), Box::new(Node::Element(4.)));
        assert_eq!(node.eval(), 12.);
    }

    #[test]
    fn divide() {
        let node = Node::Divide(Box::new(Node::Element(6.)), Box::new(Node::Element(2.)));
        assert_eq!(node.eval(), 3.);
    }

    #[test]
    fn add() {
        let node = Node::Sum(Box::new(Node::Element(3.)), Box::new(Node::Element(4.)));
        assert_eq!(node.eval(), 7.);
    }

    #[test]
    fn subtract() {
        let node = Node::Subtract(Box::new(Node::Element(3.)), Box::new(Node::Element(4.)));
        assert_eq!(node.eval(), -1.);
    }

    #[test]
    fn power() {
        let node = Node::Power(Box::new(Node::Element(3.)), Box::new(Node::Element(4.)));
        assert_eq!(node.eval(), 81.);
    }
}
