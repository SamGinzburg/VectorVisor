#[derive(Debug)]
pub enum TrapCode {
    TrapUnreachable,          // unreachable instruction
    TrapIntOverflow,          // used in convops
    TrapInvalidConversion,    // also convops
    TrapCallIndirectNotFound, // used when checking call_indirect targets
    TrapUnimplemented,        // used internally for unimplemented WASI call stubs
    TrapOutOfBounds,          // check for OOB access when using the hcall buffer
                              // (only case where we bypass the software MMU)
}

pub fn emit_trap(code: TrapCode, emit_semicolon: bool) -> String {
    let semi = if emit_semicolon { ";" } else { "" };

    match code {
        TrapCode::TrapUnreachable => {
            format!("\t*((volatile unsigned long *)0x0) = 0x0{}\n", semi)
        }
        TrapCode::TrapIntOverflow => {
            format!("\t*((volatile unsigned long *)0x1) = 0x1{}\n", semi)
        }
        TrapCode::TrapInvalidConversion => {
            format!("\t*((volatile unsigned long *)0x2) = 0x2{}\n", semi)
        }
        TrapCode::TrapCallIndirectNotFound => {
            format!("\t*((volatile unsigned long *)0x3) = 0x3{}\n", semi)
        }
        TrapCode::TrapUnimplemented => {
            format!("\t*((volatile unsigned long *)0x4) = 0x4{}\n", semi)
        }
        TrapCode::TrapUnimplemented => {
            format!("\t*((volatile unsigned long *)0x5) = 0x5{}\n", semi)
        }
        TrapCode::TrapOutOfBounds => {
            format!("\t*((volatile unsigned long *)0x6) = 0x6{}\n", semi)
        }
    }
}
