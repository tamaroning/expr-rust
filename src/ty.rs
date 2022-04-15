#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    Unit,
    Fn {
        arg_tys: Vec<Box<Type>>,
        ret_ty: Box<Type>,
    },
    Unresolved,
    NoType,
}
