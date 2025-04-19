#[derive(Debug, Clone)]
pub enum Operator {
    Walrus,
    Asign,
    Equals,
    NotEquals,
    Compare,
    Plus,
    Minus,
    Asterisk,
    Fslash,
    Percent, // Modulus,
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
}
