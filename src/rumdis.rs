/// A Umi is just a synonym for a Universal Machine instruction
type Umi = u32;

/// A Field represents some bitfield of a Umi
pub struct Field {
    width: u32,
    lsb: u32,
}

/// Registers A, B, C of a normal instruction
static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
/// Register A of a Load Value instruction
static RL: Field = Field { width: 3, lsb: 25 };
/// The value field for Load Value
static VL: Field = Field { width: 25, lsb: 0 };
/// The Opcode
static OP: Field = Field { width: 4, lsb: 28 };

/// Create a mask (all 1s) of `bits` bits
fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

/// Given `inst` (a `Umi`) pretty-print a human-readable version
pub fn disassemble(inst: Umi) -> String {
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        o if o == Opcode::Load as u32 => {
            format!(
                "r{} := m[r{}][r{}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::Store as u32 => {
            format!(
                "m[r{}][r{}] := r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::Add as u32 => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::Mul as u32 => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::Div as u32 => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        // possible enhancement: if RB == RC, complement RC
        o if o == Opcode::Nand as u32 => {
            format!(
                "r{} := r{} nand r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::Halt as u32 => {
            format!("halt")
        }
        o if o == Opcode::MapSegment as u32 => {
            format!(
                "r{} := map segment (r{} words);",
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        o if o == Opcode::UnmapSegment as u32 => {
            format!("unmap r{};", get(&RC, inst))
        }
        o if o == Opcode::Output as u32 => {
            format!("output r{};", get(&RC, inst))
        }
        o if o == Opcode::Input as u32 => {
            format!("r{} := input();", get(&RC, inst))
        }
        o if o == Opcode::LoadProgram as u32 => {
            format!(
                "goto r{} in program m[r{}];",
                get(&RC, inst),
                get(&RB, inst)
            )
        }
        o if o == Opcode::LoadValue as u32 => {
            format!("r{} := {};", get(&RL, inst), get(&VL, inst))
        }

        _ => {
            format!(".data 0x{:x}", inst)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}
