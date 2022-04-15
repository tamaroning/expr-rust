use llvm::core::*;
use llvm::prelude::*;
use llvm_sys as llvm;
use std::ffi::CString;

use crate::ast::*;
use crate::ty::Type;

type GenResult = Result<LLVMValueRef, String>;

pub struct Codegen {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl Codegen {
    unsafe fn new(mod_name: String) -> Self {
        let c_mod_name = CString::new(mod_name).unwrap();

        let context = LLVMContextCreate();
        Codegen {
            context,
            module: LLVMModuleCreateWithName(c_mod_name.as_ptr()),
            builder: LLVMCreateBuilder(),
        }
    }

    pub unsafe fn dump_module(&self) {
        LLVMDumpModule(self.module);
    }

    pub unsafe fn write_llvm_bc(&mut self) {
        let file_name = CString::new("a.bc").unwrap();
        llvm::bit_writer::LLVMWriteBitcodeToFile(self.module, file_name.as_ptr());
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
        } = func;
        let ret_llty = LLVMVoidType();
        let func: LLVMValueRef =
            LLVMAddFunction(self.module, ident_to_cstr(name).as_ptr(), ret_llty);
        // TODO: func args
        let bb_entry: LLVMBasicBlockRef =
            LLVMAppendBasicBlock(func, CString::new("entry").unwrap().as_ptr());
        LLVMPositionBuilderAtEnd(self.builder, bb_entry);
        self.gen_func_body(*body);

        Ok(func)
    }

    unsafe fn gen_func_body(&mut self, body: Expr) {
        // body corresponds to inner expr of block expr
        self.gen_expr(body);

        LLVMBuildRetVoid(self.builder);
    }

    unsafe fn gen_expr(&mut self, expr: Expr) -> GenResult {
        match expr {
            Expr::LitExpr(lit) => self.gen_lit_expr(lit),
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
        let v = LLVMConstInt(LLVMInt32Type(), 100, 0);
        Ok(v)
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
        let res = match binary {
            BinaryOpExpr::Add(lhs, rhs) => LLVMBuildAdd(
                self.builder,
                self.gen_expr(*lhs)?,
                self.gen_expr(*rhs)?,
                CString::new("add").unwrap().as_ptr(),
            ),
            BinaryOpExpr::Sub(lhs, rhs) => LLVMBuildAdd(
                self.builder,
                self.gen_expr(*lhs)?,
                self.gen_expr(*rhs)?,
                CString::new("sub").unwrap().as_ptr(),
            ),
            BinaryOpExpr::Mul(lhs, rhs) => LLVMBuildAdd(
                self.builder,
                self.gen_expr(*lhs)?,
                self.gen_expr(*rhs)?,
                CString::new("mul").unwrap().as_ptr(),
            ),
            BinaryOpExpr::Div(lhs, rhs) => LLVMBuildAdd(
                self.builder,
                self.gen_expr(*lhs)?,
                self.gen_expr(*rhs)?,
                CString::new("div").unwrap().as_ptr(),
            ),
            _ => todo!(),
        };
        Ok(res)
    }
}

pub fn codegen(program: Program) {
    let mut codegen = unsafe { Codegen::new("MyModule".to_string()) };

    unsafe {
        codegen.gen_program(program);
        codegen.dump_module();
        codegen.write_llvm_bc();
    }
}

fn ident_to_cstr(ident: Ident) -> CString {
    CString::new(ident.sym).unwrap()
}
