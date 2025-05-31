#[macro_export]
macro_rules! jit_compile_and_run {
    ($self:ident, $display:ident, $opcode:expr, $compile_fn:expr $(, $args:expr)* $(,)?) => {{
        let mut asm = dynasmrt::x64::Assembler::new().unwrap();
        let offset = $compile_fn(&mut asm, $($args), *);
        let code = asm.finalize().unwrap();

        $self.jit_cache.insert(
            $opcode,
            JitBlock {
                code,
                entry: offset.0
            },
        );

        let func: extern "C" fn(&mut CPU, &mut ContextPixels) -> bool =
            unsafe { std::mem::transmute($self.jit_cache[&$opcode].code.ptr(offset))};

        println!("COMPILE {:?}", stringify!($compile_fn));
        func($self, $display)
    }}
}
