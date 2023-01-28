use std::path::Path;

use crate::ast::{Decl, Module as PewterModule};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;

const DISPLAY_LLVM_BITCODE: bool = true;

impl Decl {
    fn codegen<'ctx>(
        &self,
        ctx: &'ctx Context,
        module: &inkwell::module::Module<'ctx>,
    ) -> inkwell::values::GlobalValue<'ctx> {
        let Decl(id, value) = self;
        let i32_t = ctx.i32_type();
        let glob = module.add_global(i32_t, None, id.as_str());
        glob.set_constant(true);
        let value = i32_t.const_int(*value as u64, true);
        glob.set_initializer(&value);
        glob
    }
}

impl PewterModule {
    pub fn codegen<'ctx>(&self, ctx: &'ctx Context, _builder: &Builder) -> LLVMModule<'ctx> {
        let llvm_module = ctx.create_module(self.name.as_str());
        for decl in &self.decls {
            decl.codegen(ctx, &llvm_module);
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
