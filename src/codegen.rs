use std::path::Path;

use crate::ast;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::IntType;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

const DISPLAY_LLVM_BITCODE: bool = true;

impl ast::Literal {
    fn codegen<'ctx>(&self, ctx: &'ctx Context) -> (IntType<'ctx>, IntValue<'ctx>) {
        match self {
            Self::Int(i) => {
                let t = ctx.i32_type();
                let v = t.const_int(*i as u64, true);
                (t, v)
            }
        }
    }
}

impl ast::Def {
    fn codegen<'ctx>(
        &self,
        ctx: &'ctx Context,
        module: &inkwell::module::Module<'ctx>,
    ) -> inkwell::values::GlobalValue<'ctx> {
        let ast::Def(id, lit) = self;
        let (t, v) = lit.codegen(ctx);
        let glob = module.add_global(t, None, id.as_str());
        glob.set_constant(true);
        glob.set_initializer(&v);
        glob
    }
}

impl ast::Compunit {
    pub fn codegen<'ctx>(&self, ctx: &'ctx Context, _builder: &Builder) -> LLVMModule<'ctx> {
        let llvm_module = ctx.create_module(self.name.as_str());
        for def in &self.defs {
            def.codegen(ctx, &llvm_module);
        }
        llvm_module
    }

    pub fn codegen_to_object_file(&self, output_path: &Path) {
        let context = Context::create();
        let builder = context.create_builder();
        let llvm_module = self.codegen(&context, &builder);

        llvm_module.verify().unwrap();

        if DISPLAY_LLVM_BITCODE {
            println!("{}", llvm_module.to_string());
        }

        let default_triple = TargetMachine::get_default_triple();
        Target::initialize_all(&InitializationConfig::default());
        let opt = OptimizationLevel::Default;
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let features = TargetMachine::get_host_cpu_features();
        let cpu = TargetMachine::get_host_cpu_name();
        let target = Target::from_triple(&default_triple).expect("target lookup failed");
        let target_machine = target
            .create_target_machine(
                &default_triple,
                cpu.to_str().unwrap(),
                features.to_str().unwrap(),
                opt,
                reloc,
                model,
            )
            .expect("target machine lookup failed");
        target_machine
            .write_to_file(&llvm_module, FileType::Object, output_path)
            .expect("failed writing to 'out.o'");
    }
}
