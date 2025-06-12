use std::sync::atomic::AtomicUsize;

use dynasmrt::DynasmApi;
use dynasmrt::DynasmLabelApi;
use dynasmrt::{dynasm, x64::Assembler, AssemblyOffset};
use memoffset::offset_of;
use rand::random;

use super::schema::CPU;

extern "C" fn panic_stack_underflow(_cpu: &mut CPU) -> ! {
    panic!("Stack underflow: retour sans appel de sous programme");
}

extern "C" fn panic_stack_overflow(_cpu: &mut CPU) -> ! {
    panic!("Stack overflow: retour sans appel de sous programme");
}

impl CPU {
    pub fn jit_compile_00EE(asm: &mut Assembler) -> AssemblyOffset {
        let offset_sp = offset_of!(CPU, sp) as i32;
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_stack = offset_of!(CPU, stack) as i32;

        let s = asm.offset();

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

            ; mov rax, 1
            ; ret

            // Label panic
            ; panic_underflow:
            // rdi contient déjà l'argument &mut CPU
            ; mov rax, QWORD panic_stack_underflow as _
            ; call rax
            ; int3
        );
        s
    }

    pub fn jit_compile_1NNN(asm: &mut Assembler, nnn: u16) -> AssemblyOffset {
        let offset_pc = offset_of!(CPU, pc) as i32;
        let s = asm.offset();
        dynasm!(asm
            ; mov WORD [rdi + offset_pc], nnn as i16
            ; mov rax, 0 // charge false en valeur de retour
            ; ret
        );
        s
    }

    pub fn jit_compile_2NNN(asm: &mut Assembler, nnn: u16) -> AssemblyOffset {
        let offset_sp = offset_of!(CPU, sp) as i32;
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_stack = offset_of!(CPU, stack) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx rcx, BYTE [rdi + offset_sp]
            ; cmp rcx, 16
            ; jae > panic_overflow

            ; mov dx, WORD [rdi + offset_pc] // dx = pc
            ; mov WORD [rdi + offset_stack + rcx * 2], dx // stack[sp] = pc

            ; inc BYTE [rdi + offset_sp] // sp += 1
            ; mov WORD [rdi + offset_pc], nnn as i16 // pc = nnn

            ; mov rax, 0
            ; ret

            ; panic_overflow:
            ; mov rax, QWORD panic_stack_overflow as _
            ; call rax
            ; int3
        );
        s
    }

    pub fn jit_compile_3XKK(asm: &mut Assembler, x: u8, kk: u8) -> AssemblyOffset {
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx eax, BYTE [rdi +  offset_v + x as i32]
            ; cmp al, kk as i8
            ; jne >skip

            ; add WORD [rdi + offset_pc], 2

            ; skip:
            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_4XKK(asm: &mut Assembler, x: u8, kk: u8) -> AssemblyOffset {
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx eax, BYTE [rdi +  offset_v + x as i32]
            ; cmp al, kk as i8
            ; je >skip

            ; add WORD [rdi + offset_pc], 2

            ; skip:
            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_5XY0(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_pc = offset_of!(CPU, pc) as i32;
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx eax, BYTE [rdi + offset_v + x as i32]
            ; mov dl, BYTE [rdi + offset_v + y as i32]
            ; cmp al, dl
            ; jne >skip

            ; add WORD [rdi + offset_pc], 2

            ; skip:
            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_6XNN(asm: &mut Assembler, x: u8, kk: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov BYTE [rdi +  offset_v + x as i32], kk as i8

            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_7XNN(asm: &mut Assembler, x: u8, kk: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; add BYTE [rdi +  offset_v + x as i32], kk as i8

            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_8XY0(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov dl, BYTE [rdi + offset_v + y as i32]
            ; mov BYTE [rdi + offset_v + x as i32], dl

            ; mov rax, 1
            ; ret
        );
        s
    }

    pub fn jit_compile_8XY1(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; or al,  BYTE [rdi + offset_v + y as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY2(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; and al, BYTE [rdi + offset_v + y as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY3(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; xor al, BYTE [rdi + offset_v + y as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY4(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; add al, BYTE [rdi + offset_v + y as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; setc dl
            ; mov BYTE [rdi + offset_v + 0xF], dl

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY5(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; sub al, BYTE [rdi + offset_v + y as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; setnc dl
            ; mov BYTE [rdi + offset_v + 0xF], dl

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY6(asm: &mut Assembler, x: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]
            ; and dl, 0
            ; mov dl, al
            ; and dl, 1
            ; mov BYTE [rdi + offset_v + 0xF], dl

            ; shr al, 1
            ; mov BYTE [rdi + offset_v + x as i32], al

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XY7(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + y as i32]
            ; sub al, BYTE [rdi + offset_v + x as i32]

            ; mov BYTE [rdi + offset_v + x as i32], al

            ; setnc dl
            ; mov BYTE [rdi + offset_v + 0xF], dl

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_8XYE(asm: &mut Assembler, x: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov al, BYTE [rdi + offset_v + x as i32]

            ; mov dl, al
            ; shr al, 7
            ; and al, 1
            ; mov BYTE [rdi + offset_v + 0xF], al

            ; shl dl, 1
            ; mov BYTE [rdi + offset_v + x as i32], dl

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_9XY0(asm: &mut Assembler, x: u8, y: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;
        let offset_pc = offset_of!(CPU, pc) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx eax, BYTE [rdi + offset_v + x as i32]
            ; mov dl, BYTE [rdi + offset_v + y as i32]
            ; cmp al, dl
            ; je >skip

            ; add WORD [rdi + offset_pc], 2

            ; skip:
            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_ANNN(asm: &mut Assembler, nnn: u16) -> AssemblyOffset {
        let offset_i = offset_of!(CPU, I) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov WORD [rdi + offset_i], nnn as i16

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_BNNN(asm: &mut Assembler, nnn: u16) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;
        let offset_pc = offset_of!(CPU, pc) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; movzx ax, BYTE [rdi + offset_v]
            ; add ax, nnn as i16

            ; add WORD [rdi + offset_pc], ax

            ; mov rax, 1
            ; ret
        );

        s
    }

    pub fn jit_compile_FX07(asm: &mut Assembler, x: u8) -> AssemblyOffset {
        let offset_v = offset_of!(CPU, V) as i32;
        let offset_gc = offset_of!(CPU, game_count) as i32;

        let s = asm.offset();

        dynasm!(asm
            ; mov dl, BYTE [rdi + offset_gc]
            ; mov BYTE [rdi + offset_v + x as i32], dl

            ; mov rax, 1
            ; ret
        );
        s
    }
}
