enum Expression {
    Felt(String),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    FunctionCall(String, Vec<Box<Expression>>)
}

struct FunctionDefinition {
    name: String, 
    arguments : Vec<String>,
    blocks : Vec<Expression>,
    returnStatement : <Expression>
}

struct VariableDefinition {
    name : String, 
    value : Expression
}