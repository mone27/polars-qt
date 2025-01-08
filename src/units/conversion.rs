use num_rational::Rational64;

struct BaseDimension{
    name: String
}


enum Operator{
    Mul,
    Div,
    Exp
}

enum Operand{
    BaseDimension(Dimension),
    Dimension(Dimension),
    Number(Rational64)
}

struct Operation{
    operator: Operator,
    left: Operand,
    right: Operand
}

struct Dimension{
    // name: String, 
    operands: Box<Operation> // Needed Box because is recursive
}

struct BaseUnit{
    name: String,
    dimension: Dimension
}

struct Unit{
    name: String,
    dimension: Dimension,
    
}
