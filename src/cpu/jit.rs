use dynasmrt::DynasmApi;
use dynasmrt::DynasmLabelApi;
use dynasmrt::{dynasm, x64::Assembler, AssemblyOffset};
use memoffset::offset_of;

use super::schema::CPU;

extern "C" fn panic_stack_underflow(cpu: &mut CPU) -> ! {
    panic!("Stack underflow: retour sans appel de sous programme");
}

impl CPU {
    pub fn jit_compile_00EE(asm: &mut Assembler) -> AssemblyOffset {
        let offset_sp = offset_of!(CPU, sp) as i32;
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_stack = offset_of!(CPU, stack) as i32;

        let s = asm.offset();

        unsafe {
            dynasm!(asm
                // Charger la valeur de sp dans al (byte)
                ; mov al, BYTE [rdi + offset_sp]

                // Tester si sp == 0
                ; test al, al
                ; jz >panic_underflow

                // Décrémenter sp
                ; dec al
                ; mov BYTE [rdi + offset_sp], al

                // Charger la valeur stack[sp] dans dx (mot 16 bits)
                ; movzx rcx, al
                ; movzx rdx, WORD [rdi + offset_stack + rcx * 2]

                // Mettre dx dans pc
                ; mov WORD [rdi + offset_pc], dx

                ; ret

                // Label panic
                ; panic_underflow:
                // rdi contient déjà l'argument &mut CPU
                ; mov rax, QWORD panic_stack_underflow as _
                ; call rax
                ; int3
            )
        }
        s
    }
}
