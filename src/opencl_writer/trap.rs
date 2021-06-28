pub enum TrapCode {
    TrapUnreachable,
    TrapIntOverflow,
    TrapInvalidConversion,
}

pub fn emit_trap(code: TrapCode, emit_semicolon: bool) -> String {
    let semi = if emit_semicolon {
        ";"
    } else {
        ""
    };

    match code {
        TrapCode::TrapUnreachable => format!("\t*((unsigned long *)0x0) = 0x42{}\n", semi),
        TrapCode::TrapIntOverflow => format!("\t*((unsigned long *)0x1) = 0x42{}\n", semi),
        TrapCode::TrapInvalidConversion => format!("\t*((unsigned long *)0x2) = 0x42{}\n", semi),
    }
}