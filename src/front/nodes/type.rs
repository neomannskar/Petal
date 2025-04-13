#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Void,
    Char,
    I32,
    I64,
    U32,
    U64,
    Str,
    // You can add more primitives if needed.
}

// A function type holds parameter and return type information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

// A struct type holds its name and a list of field names with their types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>, //  Use HashMap if field lookup is necessary later
}

/// The main type enum. It distinguishes primitive types, function types,
/// and user-defined types (or unresolved types), and serves as the fundamental type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(PrimitiveType),
    Function(FunctionType),
    Struct(StructType),
    /// A generic or custom type that might be resolved later (for example, a type alias)
    Custom(String),
}

impl Type {
    /// A helper to quickly generate a basic (primitive) type.
    pub fn basic(name: &str) -> Self {
        match name {
            "char" => Type::Primitive(PrimitiveType::Char),
            "i32" => Type::Primitive(PrimitiveType::I32),
            "i64" => Type::Primitive(PrimitiveType::I64),
            "u32" => Type::Primitive(PrimitiveType::U32),
            "u64" => Type::Primitive(PrimitiveType::U64),
            "void" => Type::Primitive(PrimitiveType::Void),
            "str" => Type::Primitive(PrimitiveType::Str),
            _ => Type::Custom(name.to_string()),
        }
    }
}
