pub enum TrapCode {
    TrapUnreachable,
    TrapIntOverflow,
    TrapInvalidConversion,
    TrapCallIndirectNotFound,
    TrapUnimplemented
}

pub fn emit_trap(code: TrapCode, emit_semicolon: bool) -> String {
    let semi = if emit_semicolon { ";" } else { "" };

    match code {
        TrapCode::TrapUnreachable => format!("\t*((volatile unsigned long *)0x0) = 0x42{}\n", semi),
        TrapCode::TrapIntOverflow => format!("\t*((volatile unsigned long *)0x1) = 0x42{}\n", semi),
        TrapCode::TrapInvalidConversion => {
            format!("\t*((volatile unsigned long *)0x2) = 0x42{}\n", semi)
        }
        TrapCode::TrapCallIndirectNotFound => {
            format!("\t*((volatile unsigned long *)0x3) = 0x42{}\n", semi)
        }
        TrapUnimplemented => {
            format!("\t*((volatile unsigned long *)0x3) = 0x42424242{}\n", semi)
        }
    }
}
