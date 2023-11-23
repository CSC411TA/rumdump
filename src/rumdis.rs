use ansi_term;
use ansi_term::Colour::{Blue, Fixed, Green, Purple, Red};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn bin_string(inst: Umi) -> String {
    let bin = format!("{:032b}", inst);
    let junk_color = Fixed(240);
    let op_color = Fixed(208);
    let a_color = Red;
    let b_color = Green;
    let c_color = Blue;
    let v_color = Purple;
    match op(inst) {
        Some(Opcode::CMov) | Some(Opcode::Load) | Some(Opcode::Store) | Some(Opcode::Add)
        | Some(Opcode::Mul) | Some(Opcode::Div) | Some(Opcode::Nand) => {
            format!(
                "{}{}{}{}{}",
                op_color.paint(&bin[0..4]),
                junk_color.paint(&bin[4..23]),
                a_color.paint(&bin[23..26]),
                b_color.paint(&bin[26..29]),
                c_color.paint(&bin[29..])
            )
        }
        Some(Opcode::Halt) => format!(
            "{}{}",
            op_color.paint(&bin[0..4]),
            junk_color.paint(&bin[4..])
        ),
        Some(Opcode::MapSegment) | Some(Opcode::LoadProgram) => {
            format!(
                "{}{}{}{}",
                op_color.paint(&bin[0..4]),
                junk_color.paint(&bin[4..26]),
                b_color.paint(&bin[26..29]),
                c_color.paint(&bin[29..])
            )
        }
        Some(Opcode::UnmapSegment) | Some(Opcode::Output) | Some(Opcode::Input) => {
            format!(
                "{}{}{}",
                op_color.paint(&bin[0..4]),
                junk_color.paint(&bin[4..29]),
                c_color.paint(&bin[29..])
            )
        }

        Some(Opcode::LoadValue) => {
            format!(
                "{}{}{}",
                op_color.paint(&bin[0..4]),
                b_color.paint(&bin[4..7]),
                v_color.paint(&bin[7..])
            )
        }

        _ => bin.to_string(),
    }
}

/// Given `inst` (a `Umi`) pretty-print a human-readable version
pub fn disassemble(inst: Umi) -> String {
    // match FromPrimitive::from_u32(get(&OP, inst)) {
    match op(inst) {
        Some(Opcode::CMov) => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::Load) => {
            format!(
                "r{} := m[r{}][r{}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Store) => {
            format!(
                "m[r{}][r{}] := r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Add) => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Mul) => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Div) => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        // possible enhancement: if RB == RC, complement RC
        Some(Opcode::Nand) => {
            format!(
                "r{} := r{} nand r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Halt) => "halt".to_string(),
        Some(Opcode::MapSegment) => {
            format!(
                "r{} := map segment (r{} words);",
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::UnmapSegment) => {
            format!("unmap r{};", get(&RC, inst))
        }
        Some(Opcode::Output) => {
            format!("output r{};", get(&RC, inst))
        }
        Some(Opcode::Input) => {
            format!("r{} := input();", get(&RC, inst))
        }
        Some(Opcode::LoadProgram) => {
            format!(
                "goto r{} in program m[r{}];",
                get(&RC, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::LoadValue) => {
            format!("r{} := {};", get(&RL, inst), get(&VL, inst))
        }

        _ => {
            format!(".data 0x{:08x}", inst)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
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
