use super::*;
use crate::ty::Type;

// Ok
type TypeckResult = Result<Type, String>;

pub trait Typeck {
    fn typeck(&mut self) -> TypeckResult;
}

impl Program {
    pub fn typeck(&mut self) -> TypeckResult {
        let Program { items } = self;
        for item in items {
            item.typeck()?;
        }
        Ok(Type::NoType)
    }
}

impl Item {
    pub fn typeck(&mut self) -> TypeckResult {
        match self {
            Item::Func(func) => {
                func.typeck()?;
            }
        }
        Ok(Type::NoType)
    }
}

// TODO: updaye func.ty
impl Func {
    pub fn typeck(&mut self) -> TypeckResult {
        // TODO: type check args
        let expected_ret_ty = &self.ret_ty;
        let actual_ret_ty = self.body.typeck()?;

        if actual_ret_ty == *expected_ret_ty {
            let fn_ty = Type::Fn {
                arg_tys: Vec::new(),
                ret_ty: Box::new(actual_ret_ty.clone()),
            };
            self.ty = fn_ty.clone();
            Ok(fn_ty)
        } else {
            Err(format!(
                "Return type: expected {:?}, but found {:?}",
                expected_ret_ty, actual_ret_ty
            ))
        }
    }
}

impl Expr {
    pub fn typeck(&mut self) -> TypeckResult {
        let ty = match &mut self.kind {
            ExprKind::LitExpr(lit_expr) => lit_expr.typeck(),
            ExprKind::BinaryOpExpr(binary_op_expr) => binary_op_expr.typeck(),
            _ => todo!(),
        }?;

        self.ty = ty.clone();
        Ok(ty)
    }
}

impl LitExpr {
    fn typeck(&mut self) -> TypeckResult {
        match self {
            LitExpr::Num(_) => Ok(Type::I32),
        }
    }
}

impl BinaryOpExpr {
    fn typeck(&mut self) -> TypeckResult {
        match self {
            BinaryOpExpr::Add(lhs, rhs)
            | BinaryOpExpr::Sub(lhs, rhs)
            | BinaryOpExpr::Mul(lhs, rhs)
            | BinaryOpExpr::Div(lhs, rhs) => match (lhs.typeck()?, rhs.typeck()?) {
                (Type::I32, Type::I32) => Ok(Type::I32),
                _ => Err("mismathched type".to_string()),
            },
        }
    }
}
