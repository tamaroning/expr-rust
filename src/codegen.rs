use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{self, IntType};
use inkwell::values::{self, BasicValue};
use std::ffi::CString;

use crate::ast::*;
use crate::lexer::Ident;
use crate::ty::Type;

type GenResult<'gen> = Result<GenValue<'gen>, String>;

enum GenValue<'gen> {
    Func(values::FunctionValue<'gen>),
    Int(values::IntValue<'gen>),
    Instr(values::InstructionValue<'gen>),
    NoValue,
}

pub fn codegen(program: Program) {
    let mut codegen = unsafe { CodeGen::new("mod_name") };
}

struct CodeGen<'ctx> {
    context: Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    unsafe fn new(mod_name: &str) -> Self {
        let context = Context::create();
        let module = context.create_module(mod_name);
        let builder = context.create_builder();
        CodeGen {
            context,
            module,
            builder,
        }
    }

    unsafe fn write_bc_to_path(&self) {
        self.module
            .write_bitcode_to_path(std::path::Path::new("a.bc"));
    }

    unsafe fn gen_program(&mut self, program: Program) -> Result<(), String> {
        for item in program.items {
            self.gen_item(item)?;
        }
        Ok(())
    }

    unsafe fn gen_item(&mut self, item: Item) -> GenResult {
        match item {
            Item::Func(func) => self.gen_func(func),
        }
    }

    unsafe fn gen_func(&mut self, func: Func) -> GenResult {
        let Func {
            name,
            args,
            ret_ty,
            body,
            ty,
        } = func;
        let ret_llty = self.context.i32_type();
        let fn_llty = ret_llty.fn_type(&[], false);
        let func = self.module.add_function(&name.sym, fn_llty, None);

        // TODO: func args
        let bb_entry = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(bb_entry);

        self.gen_func_body(*body)?;

        Ok(GenValue::Func(func))
    }

    unsafe fn gen_func_body(&mut self, body: Expr) -> GenResult {
        // body corresponds to inner expr of block expr
        let body_value = self.gen_expr(body)?;

        let ret: Option<&dyn values::BasicValue> = match &body_value {
            GenValue::NoValue => None,
            GenValue::Int(int_value) => Some(int_value),
            _ => todo!(),
        };
        self.builder.build_return(ret);

        Ok(GenValue::NoValue)
    }

    unsafe fn gen_expr(&mut self, expr: Expr) -> GenResult {
        match expr.kind {
            ExprKind::LitExpr(lit) => self.gen_lit_expr(lit),
            _ => todo!()
            //Expr::IdentExpr(ident) => self.gen_ident_expr(ident),
            //Expr::UnaryOpExpr(unary) => self.gen_unary_op_expr(unary),
            //Expr::BinaryOpExpr(binary) => self.gen_binary_op_expr(binary),
        }
    }

    unsafe fn gen_lit_expr(&mut self, lit: LitExpr) -> GenResult {
        match lit {
            LitExpr::Num(n) => self.make_num(n),
        }
    }

    unsafe fn make_num(&mut self, n: i32) -> GenResult {
        // TODO: what should i pass to 2nd arg (unsigned long long) ?
        let v = self.context.i32_type().const_int(1 as u64, false);
        Ok(GenValue::Int(v))
    }

    /*
    fn gen_ident_expr(&mut self, ident: Ident) -> GenResult {
        Ok(())
    }

    fn gen_unary_op_expr(&mut self, ident: UnaryOpExpr) -> GenResult {
        Ok(())
    }
    */

    unsafe fn gen_binary_op_expr(&mut self, binary: BinaryOpExpr) -> GenResult {
        let (lhs, rhs) = match &binary {
            BinaryOpExpr::Add(lhs, rhs) => (&**lhs, &**rhs),
            BinaryOpExpr::Sub(lhs, rhs) => (&**lhs, &**rhs),
            BinaryOpExpr::Mul(lhs, rhs) => (&**lhs, &**rhs),
            BinaryOpExpr::Div(lhs, rhs) => (&**lhs, &**rhs),
        };

        let (lhs_gen_val, rhs_gen_val) = (self.gen_expr(*lhs)?, self.gen_expr(*rhs)?);

        let ret = match (lhs_gen_val, rhs_gen_val) {
            (GenValue::Int(lhs_int_value), GenValue::Int(rhs_int_value)) => {
                let int_value = self
                    .builder
                    .build_int_add(lhs_int_value, rhs_int_value, "add");
                GenValue::Int(int_value)
            }
            _ => todo!(),
        };

        Ok(ret)
    }
}

fn ident_to_cstr(ident: Ident) -> CString {
    CString::new(ident.sym).unwrap()
}
