use leb128;

// https://webassembly.github.io/spec/core/binary/modules.html#sections
enum Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Func = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
}

// https://webassembly.github.io/spec/core/binary/types.html
enum Valtype {
    I32 = 0x7f,
    F32 = 0x7d,
}

// https://webassembly.github.io/spec/core/binary/types.html#binary-blocktype
enum Blocktype {
    Void = 0x40,
}

// https://webassembly.github.io/spec/core/binary/instructions.html
enum Opcodes {
    Block = 0x02,
    Loop = 0x03,
    Br = 0x0c,
    BrIf = 0x0d,
    End = 0x0b,
    Call = 0x10,
    GetLocal = 0x20,
    SetLocal = 0x21,
    I32Store8 = 0x3a,
    I32Const = 0x41,
    F32Const = 0x43,
    I32Eqz = 0x45,
    I32Eq = 0x46,
    F32Eq = 0x5b,
    F32Lt = 0x5d,
    F32Gt = 0x5e,
    I32And = 0x71,
    F32Add = 0x92,
    F32Sub = 0x93,
    F32Mul = 0x94,
    F32Div = 0x95,
    I32truncF32s = 0xa8,
}

// http://webassembly.github.io/spec/core/binary/modules.html#export-section
enum ExportType {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

// http://webassembly.github.io/spec/core/binary/types.html#function-types
const FUNCTION_TYPE: usize = 0x60;

const EMPTY_ARRAY: usize = 0x0;

// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
const MAGIC_MODULE_HEADER: [usize; 4] = [0x00, 0x61, 0x73, 0x6d];
const MODULE_VERSION: [usize; 4] = [0x01, 0x00, 0x00, 0x00];

fn unsigned_led128(value: u64) -> Vec<u8> {
    let mut result = vec![];
    leb128::write::unsigned(&mut result, value).expect("Should write number");
    result
}

// https://webassembly.github.io/spec/core/binary/conventions.html#binary-vec
// Vectors are encoded with their length followed by their element sequence
fn encode_vector(data: Vec<u8>) -> Vec<u8> {
    [unsigned_led128(data.len() as u64), data].concat()
}

// fn create_section<T>(section_type: Section, data: T) -> Vec<u8> {}
