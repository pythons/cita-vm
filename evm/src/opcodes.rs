use std::convert::From;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0a,
    SIGNEXTEND = 0x0b,
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,
    SHA3 = 0x20,
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    EXTCODEHASH = 0x3f,
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5a,
    JUMPDEST = 0x5b,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6a,
    PUSH12 = 0x6b,
    PUSH13 = 0x6c,
    PUSH14 = 0x6d,
    PUSH15 = 0x6e,
    PUSH16 = 0x6f,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7a,
    PUSH28 = 0x7b,
    PUSH29 = 0x7c,
    PUSH30 = 0x7d,
    PUSH31 = 0x7e,
    PUSH32 = 0x7f,
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8a,
    DUP12 = 0x8b,
    DUP13 = 0x8c,
    DUP14 = 0x8d,
    DUP15 = 0x8e,
    DUP16 = 0x8f,
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9a,
    SWAP12 = 0x9b,
    SWAP13 = 0x9c,
    SWAP14 = 0x9d,
    SWAP15 = 0x9e,
    SWAP16 = 0x9f,
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,
    LOG4 = 0xa4,
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,
    REVERT = 0xfd,
    STATICCALL = 0xfa,
    SUICIDE = 0xff,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            OpCode::STOP => write!(f, "STOP"),
            OpCode::ADD => write!(f, "ADD"),
            OpCode::MUL => write!(f, "MUL"),
            OpCode::SUB => write!(f, "SUB"),
            OpCode::DIV => write!(f, "DIV"),
            OpCode::SDIV => write!(f, "SDIV"),
            OpCode::MOD => write!(f, "MOD"),
            OpCode::SMOD => write!(f, "SMOD"),
            OpCode::ADDMOD => write!(f, "ADDMOD"),
            OpCode::MULMOD => write!(f, "MULMOD"),
            OpCode::EXP => write!(f, "EXP"),
            OpCode::SIGNEXTEND => write!(f, "SIGNEXTEND"),
            OpCode::LT => write!(f, "LT"),
            OpCode::GT => write!(f, "GT"),
            OpCode::SLT => write!(f, "SLT"),
            OpCode::SGT => write!(f, "SGT"),
            OpCode::EQ => write!(f, "EQ"),
            OpCode::ISZERO => write!(f, "ISZERO"),
            OpCode::AND => write!(f, "AND"),
            OpCode::OR => write!(f, "OR"),
            OpCode::XOR => write!(f, "XOR"),
            OpCode::NOT => write!(f, "NOT"),
            OpCode::BYTE => write!(f, "BYTE"),
            OpCode::SHL => write!(f, "SHL"),
            OpCode::SHR => write!(f, "SHR"),
            OpCode::SAR => write!(f, "SAR"),
            OpCode::SHA3 => write!(f, "SHA3"),
            OpCode::ADDRESS => write!(f, "ADDRESS"),
            OpCode::BALANCE => write!(f, "BALANCE"),
            OpCode::ORIGIN => write!(f, "ORIGIN"),
            OpCode::CALLER => write!(f, "CALLER"),
            OpCode::CALLVALUE => write!(f, "CALLVALUE"),
            OpCode::CALLDATALOAD => write!(f, "CALLDATALOAD"),
            OpCode::CALLDATASIZE => write!(f, "CALLDATASIZE"),
            OpCode::CALLDATACOPY => write!(f, "CALLDATACOPY"),
            OpCode::CODESIZE => write!(f, "CODESIZE"),
            OpCode::CODECOPY => write!(f, "CODECOPY"),
            OpCode::GASPRICE => write!(f, "GASPRICE"),
            OpCode::EXTCODESIZE => write!(f, "EXTCODESIZE"),
            OpCode::EXTCODECOPY => write!(f, "EXTCODECOPY"),
            OpCode::RETURNDATASIZE => write!(f, "RETURNDATASIZE"),
            OpCode::RETURNDATACOPY => write!(f, "RETURNDATACOPY"),
            OpCode::EXTCODEHASH => write!(f, "EXTCODEHASH"),
            OpCode::BLOCKHASH => write!(f, "BLOCKHASH"),
            OpCode::COINBASE => write!(f, "COINBASE"),
            OpCode::TIMESTAMP => write!(f, "TIMESTAMP"),
            OpCode::NUMBER => write!(f, "NUMBER"),
            OpCode::DIFFICULTY => write!(f, "DIFFICULTY"),
            OpCode::GASLIMIT => write!(f, "GASLIMIT"),
            OpCode::POP => write!(f, "POP"),
            OpCode::MLOAD => write!(f, "MLOAD"),
            OpCode::MSTORE => write!(f, "MSTORE"),
            OpCode::MSTORE8 => write!(f, "MSTORE8"),
            OpCode::SLOAD => write!(f, "SLOAD"),
            OpCode::SSTORE => write!(f, "SSTORE"),
            OpCode::JUMP => write!(f, "JUMP"),
            OpCode::JUMPI => write!(f, "JUMPI"),
            OpCode::PC => write!(f, "PC"),
            OpCode::MSIZE => write!(f, "MSIZE"),
            OpCode::GAS => write!(f, "GAS"),
            OpCode::JUMPDEST => write!(f, "JUMPDEST"),
            OpCode::PUSH1 => write!(f, "PUSH1"),
            OpCode::PUSH2 => write!(f, "PUSH2"),
            OpCode::PUSH3 => write!(f, "PUSH3"),
            OpCode::PUSH4 => write!(f, "PUSH4"),
            OpCode::PUSH5 => write!(f, "PUSH5"),
            OpCode::PUSH6 => write!(f, "PUSH6"),
            OpCode::PUSH7 => write!(f, "PUSH7"),
            OpCode::PUSH8 => write!(f, "PUSH8"),
            OpCode::PUSH9 => write!(f, "PUSH9"),
            OpCode::PUSH10 => write!(f, "PUSH10"),
            OpCode::PUSH11 => write!(f, "PUSH11"),
            OpCode::PUSH12 => write!(f, "PUSH12"),
            OpCode::PUSH13 => write!(f, "PUSH13"),
            OpCode::PUSH14 => write!(f, "PUSH14"),
            OpCode::PUSH15 => write!(f, "PUSH15"),
            OpCode::PUSH16 => write!(f, "PUSH16"),
            OpCode::PUSH17 => write!(f, "PUSH17"),
            OpCode::PUSH18 => write!(f, "PUSH18"),
            OpCode::PUSH19 => write!(f, "PUSH19"),
            OpCode::PUSH20 => write!(f, "PUSH20"),
            OpCode::PUSH21 => write!(f, "PUSH21"),
            OpCode::PUSH22 => write!(f, "PUSH22"),
            OpCode::PUSH23 => write!(f, "PUSH23"),
            OpCode::PUSH24 => write!(f, "PUSH24"),
            OpCode::PUSH25 => write!(f, "PUSH25"),
            OpCode::PUSH26 => write!(f, "PUSH26"),
            OpCode::PUSH27 => write!(f, "PUSH27"),
            OpCode::PUSH28 => write!(f, "PUSH28"),
            OpCode::PUSH29 => write!(f, "PUSH29"),
            OpCode::PUSH30 => write!(f, "PUSH30"),
            OpCode::PUSH31 => write!(f, "PUSH31"),
            OpCode::PUSH32 => write!(f, "PUSH32"),
            OpCode::DUP1 => write!(f, "DUP1"),
            OpCode::DUP2 => write!(f, "DUP2"),
            OpCode::DUP3 => write!(f, "DUP3"),
            OpCode::DUP4 => write!(f, "DUP4"),
            OpCode::DUP5 => write!(f, "DUP5"),
            OpCode::DUP6 => write!(f, "DUP6"),
            OpCode::DUP7 => write!(f, "DUP7"),
            OpCode::DUP8 => write!(f, "DUP8"),
            OpCode::DUP9 => write!(f, "DUP9"),
            OpCode::DUP10 => write!(f, "DUP10"),
            OpCode::DUP11 => write!(f, "DUP11"),
            OpCode::DUP12 => write!(f, "DUP12"),
            OpCode::DUP13 => write!(f, "DUP13"),
            OpCode::DUP14 => write!(f, "DUP14"),
            OpCode::DUP15 => write!(f, "DUP15"),
            OpCode::DUP16 => write!(f, "DUP16"),
            OpCode::SWAP1 => write!(f, "SWAP1"),
            OpCode::SWAP2 => write!(f, "SWAP2"),
            OpCode::SWAP3 => write!(f, "SWAP3"),
            OpCode::SWAP4 => write!(f, "SWAP4"),
            OpCode::SWAP5 => write!(f, "SWAP5"),
            OpCode::SWAP6 => write!(f, "SWAP6"),
            OpCode::SWAP7 => write!(f, "SWAP7"),
            OpCode::SWAP8 => write!(f, "SWAP8"),
            OpCode::SWAP9 => write!(f, "SWAP9"),
            OpCode::SWAP10 => write!(f, "SWAP10"),
            OpCode::SWAP11 => write!(f, "SWAP11"),
            OpCode::SWAP12 => write!(f, "SWAP12"),
            OpCode::SWAP13 => write!(f, "SWAP13"),
            OpCode::SWAP14 => write!(f, "SWAP14"),
            OpCode::SWAP15 => write!(f, "SWAP15"),
            OpCode::SWAP16 => write!(f, "SWAP16"),
            OpCode::LOG0 => write!(f, "LOG0"),
            OpCode::LOG1 => write!(f, "LOG1"),
            OpCode::LOG2 => write!(f, "LOG2"),
            OpCode::LOG3 => write!(f, "LOG3"),
            OpCode::LOG4 => write!(f, "LOG4"),
            OpCode::CREATE => write!(f, "CREATE"),
            OpCode::CALL => write!(f, "CALL"),
            OpCode::CALLCODE => write!(f, "CALLCODE"),
            OpCode::RETURN => write!(f, "RETURN"),
            OpCode::DELEGATECALL => write!(f, "DELEGATECALL"),
            OpCode::CREATE2 => write!(f, "CREATE2"),
            OpCode::REVERT => write!(f, "REVERT"),
            OpCode::STATICCALL => write!(f, "STATICCALL"),
            OpCode::SUICIDE => write!(f, "SUICIDE"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(n: u8) -> OpCode {
        match n {
            0x00 => OpCode::STOP,
            0x01 => OpCode::ADD,
            0x02 => OpCode::MUL,
            0x03 => OpCode::SUB,
            0x04 => OpCode::DIV,
            0x05 => OpCode::SDIV,
            0x06 => OpCode::MOD,
            0x07 => OpCode::SMOD,
            0x08 => OpCode::ADDMOD,
            0x09 => OpCode::MULMOD,
            0x0a => OpCode::EXP,
            0x0b => OpCode::SIGNEXTEND,
            0x10 => OpCode::LT,
            0x11 => OpCode::GT,
            0x12 => OpCode::SLT,
            0x13 => OpCode::SGT,
            0x14 => OpCode::EQ,
            0x15 => OpCode::ISZERO,
            0x16 => OpCode::AND,
            0x17 => OpCode::OR,
            0x18 => OpCode::XOR,
            0x19 => OpCode::NOT,
            0x1a => OpCode::BYTE,
            0x1b => OpCode::SHL,
            0x1c => OpCode::SHR,
            0x1d => OpCode::SAR,
            0x20 => OpCode::SHA3,
            0x30 => OpCode::ADDRESS,
            0x31 => OpCode::BALANCE,
            0x32 => OpCode::ORIGIN,
            0x33 => OpCode::CALLER,
            0x34 => OpCode::CALLVALUE,
            0x35 => OpCode::CALLDATALOAD,
            0x36 => OpCode::CALLDATASIZE,
            0x37 => OpCode::CALLDATACOPY,
            0x38 => OpCode::CODESIZE,
            0x39 => OpCode::CODECOPY,
            0x3a => OpCode::GASPRICE,
            0x3b => OpCode::EXTCODESIZE,
            0x3c => OpCode::EXTCODECOPY,
            0x3d => OpCode::RETURNDATASIZE,
            0x3e => OpCode::RETURNDATACOPY,
            0x3f => OpCode::EXTCODEHASH,
            0x40 => OpCode::BLOCKHASH,
            0x41 => OpCode::COINBASE,
            0x42 => OpCode::TIMESTAMP,
            0x43 => OpCode::NUMBER,
            0x44 => OpCode::DIFFICULTY,
            0x45 => OpCode::GASLIMIT,
            0x50 => OpCode::POP,
            0x51 => OpCode::MLOAD,
            0x52 => OpCode::MSTORE,
            0x53 => OpCode::MSTORE8,
            0x54 => OpCode::SLOAD,
            0x55 => OpCode::SSTORE,
            0x56 => OpCode::JUMP,
            0x57 => OpCode::JUMPI,
            0x58 => OpCode::PC,
            0x59 => OpCode::MSIZE,
            0x5a => OpCode::GAS,
            0x5b => OpCode::JUMPDEST,
            0x60 => OpCode::PUSH1,
            0x61 => OpCode::PUSH2,
            0x62 => OpCode::PUSH3,
            0x63 => OpCode::PUSH4,
            0x64 => OpCode::PUSH5,
            0x65 => OpCode::PUSH6,
            0x66 => OpCode::PUSH7,
            0x67 => OpCode::PUSH8,
            0x68 => OpCode::PUSH9,
            0x69 => OpCode::PUSH10,
            0x6a => OpCode::PUSH11,
            0x6b => OpCode::PUSH12,
            0x6c => OpCode::PUSH13,
            0x6d => OpCode::PUSH14,
            0x6e => OpCode::PUSH15,
            0x6f => OpCode::PUSH16,
            0x70 => OpCode::PUSH17,
            0x71 => OpCode::PUSH18,
            0x72 => OpCode::PUSH19,
            0x73 => OpCode::PUSH20,
            0x74 => OpCode::PUSH21,
            0x75 => OpCode::PUSH22,
            0x76 => OpCode::PUSH23,
            0x77 => OpCode::PUSH24,
            0x78 => OpCode::PUSH25,
            0x79 => OpCode::PUSH26,
            0x7a => OpCode::PUSH27,
            0x7b => OpCode::PUSH28,
            0x7c => OpCode::PUSH29,
            0x7d => OpCode::PUSH30,
            0x7e => OpCode::PUSH31,
            0x7f => OpCode::PUSH32,
            0x80 => OpCode::DUP1,
            0x81 => OpCode::DUP2,
            0x82 => OpCode::DUP3,
            0x83 => OpCode::DUP4,
            0x84 => OpCode::DUP5,
            0x85 => OpCode::DUP6,
            0x86 => OpCode::DUP7,
            0x87 => OpCode::DUP8,
            0x88 => OpCode::DUP9,
            0x89 => OpCode::DUP10,
            0x8a => OpCode::DUP11,
            0x8b => OpCode::DUP12,
            0x8c => OpCode::DUP13,
            0x8d => OpCode::DUP14,
            0x8e => OpCode::DUP15,
            0x8f => OpCode::DUP16,
            0x90 => OpCode::SWAP1,
            0x91 => OpCode::SWAP2,
            0x92 => OpCode::SWAP3,
            0x93 => OpCode::SWAP4,
            0x94 => OpCode::SWAP5,
            0x95 => OpCode::SWAP6,
            0x96 => OpCode::SWAP7,
            0x97 => OpCode::SWAP8,
            0x98 => OpCode::SWAP9,
            0x99 => OpCode::SWAP10,
            0x9a => OpCode::SWAP11,
            0x9b => OpCode::SWAP12,
            0x9c => OpCode::SWAP13,
            0x9d => OpCode::SWAP14,
            0x9e => OpCode::SWAP15,
            0x9f => OpCode::SWAP16,
            0xa0 => OpCode::LOG0,
            0xa1 => OpCode::LOG1,
            0xa2 => OpCode::LOG2,
            0xa3 => OpCode::LOG3,
            0xa4 => OpCode::LOG4,
            0xf0 => OpCode::CREATE,
            0xf1 => OpCode::CALL,
            0xf2 => OpCode::CALLCODE,
            0xf3 => OpCode::RETURN,
            0xf4 => OpCode::DELEGATECALL,
            0xf5 => OpCode::CREATE2,
            0xfd => OpCode::REVERT,
            0xfa => OpCode::STATICCALL,
            0xff => OpCode::SUICIDE,
            _ => panic!(""),
        }
    }
}

impl From<&str> for OpCode {
    fn from(n: &str) -> OpCode {
        match n {
            "STOP" => OpCode::STOP,
            "ADD" => OpCode::ADD,
            "MUL" => OpCode::MUL,
            "SUB" => OpCode::SUB,
            "DIV" => OpCode::DIV,
            "SDIV" => OpCode::SDIV,
            "MOD" => OpCode::MOD,
            "SMOD" => OpCode::SMOD,
            "ADDMOD" => OpCode::ADDMOD,
            "MULMOD" => OpCode::MULMOD,
            "EXP" => OpCode::EXP,
            "SIGNEXTEND" => OpCode::SIGNEXTEND,
            "LT" => OpCode::LT,
            "GT" => OpCode::GT,
            "SLT" => OpCode::SLT,
            "SGT" => OpCode::SGT,
            "EQ" => OpCode::EQ,
            "ISZERO" => OpCode::ISZERO,
            "AND" => OpCode::AND,
            "OR" => OpCode::OR,
            "XOR" => OpCode::XOR,
            "NOT" => OpCode::NOT,
            "BYTE" => OpCode::BYTE,
            "SHL" => OpCode::SHL,
            "SHR" => OpCode::SHR,
            "SAR" => OpCode::SAR,
            "SHA3" => OpCode::SHA3,
            "ADDRESS" => OpCode::ADDRESS,
            "BALANCE" => OpCode::BALANCE,
            "ORIGIN" => OpCode::ORIGIN,
            "CALLER" => OpCode::CALLER,
            "CALLVALUE" => OpCode::CALLVALUE,
            "CALLDATALOAD" => OpCode::CALLDATALOAD,
            "CALLDATASIZE" => OpCode::CALLDATASIZE,
            "CALLDATACOPY" => OpCode::CALLDATACOPY,
            "CODESIZE" => OpCode::CODESIZE,
            "CODECOPY" => OpCode::CODECOPY,
            "GASPRICE" => OpCode::GASPRICE,
            "EXTCODESIZE" => OpCode::EXTCODESIZE,
            "EXTCODECOPY" => OpCode::EXTCODECOPY,
            "RETURNDATASIZE" => OpCode::RETURNDATASIZE,
            "RETURNDATACOPY" => OpCode::RETURNDATACOPY,
            "EXTCODEHASH" => OpCode::EXTCODEHASH,
            "BLOCKHASH" => OpCode::BLOCKHASH,
            "COINBASE" => OpCode::COINBASE,
            "TIMESTAMP" => OpCode::TIMESTAMP,
            "NUMBER" => OpCode::NUMBER,
            "DIFFICULTY" => OpCode::DIFFICULTY,
            "GASLIMIT" => OpCode::GASLIMIT,
            "POP" => OpCode::POP,
            "MLOAD" => OpCode::MLOAD,
            "MSTORE" => OpCode::MSTORE,
            "MSTORE8" => OpCode::MSTORE8,
            "SLOAD" => OpCode::SLOAD,
            "SSTORE" => OpCode::SSTORE,
            "JUMP" => OpCode::JUMP,
            "JUMPI" => OpCode::JUMPI,
            "PC" => OpCode::PC,
            "MSIZE" => OpCode::MSIZE,
            "GAS" => OpCode::GAS,
            "JUMPDEST" => OpCode::JUMPDEST,
            "PUSH1" => OpCode::PUSH1,
            "PUSH2" => OpCode::PUSH2,
            "PUSH3" => OpCode::PUSH3,
            "PUSH4" => OpCode::PUSH4,
            "PUSH5" => OpCode::PUSH5,
            "PUSH6" => OpCode::PUSH6,
            "PUSH7" => OpCode::PUSH7,
            "PUSH8" => OpCode::PUSH8,
            "PUSH9" => OpCode::PUSH9,
            "PUSH10" => OpCode::PUSH10,
            "PUSH11" => OpCode::PUSH11,
            "PUSH12" => OpCode::PUSH12,
            "PUSH13" => OpCode::PUSH13,
            "PUSH14" => OpCode::PUSH14,
            "PUSH15" => OpCode::PUSH15,
            "PUSH16" => OpCode::PUSH16,
            "PUSH17" => OpCode::PUSH17,
            "PUSH18" => OpCode::PUSH18,
            "PUSH19" => OpCode::PUSH19,
            "PUSH20" => OpCode::PUSH20,
            "PUSH21" => OpCode::PUSH21,
            "PUSH22" => OpCode::PUSH22,
            "PUSH23" => OpCode::PUSH23,
            "PUSH24" => OpCode::PUSH24,
            "PUSH25" => OpCode::PUSH25,
            "PUSH26" => OpCode::PUSH26,
            "PUSH27" => OpCode::PUSH27,
            "PUSH28" => OpCode::PUSH28,
            "PUSH29" => OpCode::PUSH29,
            "PUSH30" => OpCode::PUSH30,
            "PUSH31" => OpCode::PUSH31,
            "PUSH32" => OpCode::PUSH32,
            "DUP1" => OpCode::DUP1,
            "DUP2" => OpCode::DUP2,
            "DUP3" => OpCode::DUP3,
            "DUP4" => OpCode::DUP4,
            "DUP5" => OpCode::DUP5,
            "DUP6" => OpCode::DUP6,
            "DUP7" => OpCode::DUP7,
            "DUP8" => OpCode::DUP8,
            "DUP9" => OpCode::DUP9,
            "DUP10" => OpCode::DUP10,
            "DUP11" => OpCode::DUP11,
            "DUP12" => OpCode::DUP12,
            "DUP13" => OpCode::DUP13,
            "DUP14" => OpCode::DUP14,
            "DUP15" => OpCode::DUP15,
            "DUP16" => OpCode::DUP16,
            "SWAP1" => OpCode::SWAP1,
            "SWAP2" => OpCode::SWAP2,
            "SWAP3" => OpCode::SWAP3,
            "SWAP4" => OpCode::SWAP4,
            "SWAP5" => OpCode::SWAP5,
            "SWAP6" => OpCode::SWAP6,
            "SWAP7" => OpCode::SWAP7,
            "SWAP8" => OpCode::SWAP8,
            "SWAP9" => OpCode::SWAP9,
            "SWAP10" => OpCode::SWAP10,
            "SWAP11" => OpCode::SWAP11,
            "SWAP12" => OpCode::SWAP12,
            "SWAP13" => OpCode::SWAP13,
            "SWAP14" => OpCode::SWAP14,
            "SWAP15" => OpCode::SWAP15,
            "SWAP16" => OpCode::SWAP16,
            "LOG0" => OpCode::LOG0,
            "LOG1" => OpCode::LOG1,
            "LOG2" => OpCode::LOG2,
            "LOG3" => OpCode::LOG3,
            "LOG4" => OpCode::LOG4,
            "CREATE" => OpCode::CREATE,
            "CALL" => OpCode::CALL,
            "CALLCODE" => OpCode::CALLCODE,
            "RETURN" => OpCode::RETURN,
            "DELEGATECALL" => OpCode::DELEGATECALL,
            "CREATE2" => OpCode::CREATE2,
            "REVERT" => OpCode::REVERT,
            "STATICCALL" => OpCode::STATICCALL,
            "SUICIDE" => OpCode::SUICIDE,
            _ => panic!(""),
        }
    }
}


#[derive(PartialEq, Clone, Copy)]
pub enum GasPriceTier {
	Zero,
	Base,
	VeryLow,
	Low,
	Mid,
	High,
	Ext,
	Special,
}

impl GasPriceTier {
	pub fn idx(&self) -> usize {
		match self {
			&GasPriceTier::Zero => 0,
			&GasPriceTier::Base => 1,
			&GasPriceTier::VeryLow => 2,
			&GasPriceTier::Low => 3,
			&GasPriceTier::Mid => 4,
			&GasPriceTier::High => 5,
			&GasPriceTier::Ext => 6,
			&GasPriceTier::Special => 7,
		}
	}
}

impl OpCode {
    pub fn gas_price_tier(&self) -> GasPriceTier {
        match self{
            OpCode::STOP => GasPriceTier::Zero,
            OpCode::ADD => GasPriceTier::VeryLow,
            OpCode::SUB => GasPriceTier::VeryLow,
            OpCode::MUL => GasPriceTier::Low,
            OpCode::DIV => GasPriceTier::Low,
            OpCode::SDIV => GasPriceTier::Low,
            OpCode::MOD => GasPriceTier::Low,
            OpCode::SMOD => GasPriceTier::Low,
            OpCode::EXP => GasPriceTier::Special,
            OpCode::NOT => GasPriceTier::VeryLow,
            OpCode::LT => GasPriceTier::VeryLow,
            OpCode::GT => GasPriceTier::VeryLow,
            OpCode::SLT => GasPriceTier::VeryLow,
            OpCode::SGT => GasPriceTier::VeryLow,
            OpCode::EQ => GasPriceTier::VeryLow,
            OpCode::ISZERO => GasPriceTier::VeryLow,
            OpCode::AND => GasPriceTier::VeryLow,
            OpCode::OR => GasPriceTier::VeryLow,
            OpCode::XOR => GasPriceTier::VeryLow,
            OpCode::BYTE => GasPriceTier::VeryLow,
            OpCode::SHL => GasPriceTier::VeryLow,
            OpCode::SHR => GasPriceTier::VeryLow,
            OpCode::SAR => GasPriceTier::VeryLow,
            OpCode::ADDMOD => GasPriceTier::Mid,
            OpCode::MULMOD => GasPriceTier::Mid,
            OpCode::SIGNEXTEND => GasPriceTier::Low,
            OpCode::RETURNDATASIZE => GasPriceTier::Base,
            OpCode::RETURNDATACOPY => GasPriceTier::VeryLow,
            OpCode::SHA3 => GasPriceTier::Special,
            OpCode::ADDRESS => GasPriceTier::Base,
            OpCode::BALANCE => GasPriceTier::Special,
            OpCode::ORIGIN => GasPriceTier::Base,
            OpCode::CALLER => GasPriceTier::Base,
            OpCode::CALLVALUE => GasPriceTier::Base,
            OpCode::CALLDATALOAD => GasPriceTier::VeryLow,
            OpCode::CALLDATASIZE => GasPriceTier::Base,
            OpCode::CALLDATACOPY => GasPriceTier::VeryLow,
            OpCode::EXTCODEHASH => GasPriceTier::Special,
            OpCode::CODESIZE => GasPriceTier::Base,
            OpCode::CODECOPY => GasPriceTier::VeryLow,
            OpCode::GASPRICE => GasPriceTier::Base,
            OpCode::EXTCODESIZE => GasPriceTier::Special,
            OpCode::EXTCODECOPY => GasPriceTier::Special,
            OpCode::BLOCKHASH => GasPriceTier::Ext,
            OpCode::COINBASE => GasPriceTier::Base,
            OpCode::TIMESTAMP => GasPriceTier::Base,
            OpCode::NUMBER => GasPriceTier::Base,
            OpCode::DIFFICULTY => GasPriceTier::Base,
            OpCode::GASLIMIT => GasPriceTier::Base,
            OpCode::POP => GasPriceTier::Base,
            OpCode::MLOAD => GasPriceTier::VeryLow,
            OpCode::MSTORE => GasPriceTier::VeryLow,
            OpCode::MSTORE8 => GasPriceTier::VeryLow,
            OpCode::SLOAD => GasPriceTier::Special,
            OpCode::SSTORE => GasPriceTier::Special,
            OpCode::JUMP => GasPriceTier::Mid,
            OpCode::JUMPI => GasPriceTier::High,
            OpCode::PC => GasPriceTier::Base,
            OpCode::MSIZE => GasPriceTier::Base,
            OpCode::GAS => GasPriceTier::Base,
            OpCode::JUMPDEST => GasPriceTier::Special,
            OpCode::PUSH1 => GasPriceTier::VeryLow,
            OpCode::PUSH2 => GasPriceTier::VeryLow,
            OpCode::PUSH3 => GasPriceTier::VeryLow,
            OpCode::PUSH4 => GasPriceTier::VeryLow,
            OpCode::PUSH5 => GasPriceTier::VeryLow,
            OpCode::PUSH6 => GasPriceTier::VeryLow,
            OpCode::PUSH7 => GasPriceTier::VeryLow,
            OpCode::PUSH8 => GasPriceTier::VeryLow,
            OpCode::PUSH9 => GasPriceTier::VeryLow,
            OpCode::PUSH10 => GasPriceTier::VeryLow,
            OpCode::PUSH11 => GasPriceTier::VeryLow,
            OpCode::PUSH12 => GasPriceTier::VeryLow,
            OpCode::PUSH13 => GasPriceTier::VeryLow,
            OpCode::PUSH14 => GasPriceTier::VeryLow,
            OpCode::PUSH15 => GasPriceTier::VeryLow,
            OpCode::PUSH16 => GasPriceTier::VeryLow,
            OpCode::PUSH17 => GasPriceTier::VeryLow,
            OpCode::PUSH18 => GasPriceTier::VeryLow,
            OpCode::PUSH19 => GasPriceTier::VeryLow,
            OpCode::PUSH20 => GasPriceTier::VeryLow,
            OpCode::PUSH21 => GasPriceTier::VeryLow,
            OpCode::PUSH22 => GasPriceTier::VeryLow,
            OpCode::PUSH23 => GasPriceTier::VeryLow,
            OpCode::PUSH24 => GasPriceTier::VeryLow,
            OpCode::PUSH25 => GasPriceTier::VeryLow,
            OpCode::PUSH26 => GasPriceTier::VeryLow,
            OpCode::PUSH27 => GasPriceTier::VeryLow,
            OpCode::PUSH28 => GasPriceTier::VeryLow,
            OpCode::PUSH29 => GasPriceTier::VeryLow,
            OpCode::PUSH30 => GasPriceTier::VeryLow,
            OpCode::PUSH31 => GasPriceTier::VeryLow,
            OpCode::PUSH32 => GasPriceTier::VeryLow,
            OpCode::DUP1 => GasPriceTier::VeryLow,
            OpCode::DUP2 => GasPriceTier::VeryLow,
            OpCode::DUP3 => GasPriceTier::VeryLow,
            OpCode::DUP4 => GasPriceTier::VeryLow,
            OpCode::DUP5 => GasPriceTier::VeryLow,
            OpCode::DUP6 => GasPriceTier::VeryLow,
            OpCode::DUP7 => GasPriceTier::VeryLow,
            OpCode::DUP8 => GasPriceTier::VeryLow,
            OpCode::DUP9 => GasPriceTier::VeryLow,
            OpCode::DUP10 => GasPriceTier::VeryLow,
            OpCode::DUP11 => GasPriceTier::VeryLow,
            OpCode::DUP12 => GasPriceTier::VeryLow,
            OpCode::DUP13 => GasPriceTier::VeryLow,
            OpCode::DUP14 => GasPriceTier::VeryLow,
            OpCode::DUP15 => GasPriceTier::VeryLow,
            OpCode::DUP16 => GasPriceTier::VeryLow,
            OpCode::SWAP1 => GasPriceTier::VeryLow,
            OpCode::SWAP2 => GasPriceTier::VeryLow,
            OpCode::SWAP3 => GasPriceTier::VeryLow,
            OpCode::SWAP4 => GasPriceTier::VeryLow,
            OpCode::SWAP5 => GasPriceTier::VeryLow,
            OpCode::SWAP6 => GasPriceTier::VeryLow,
            OpCode::SWAP7 => GasPriceTier::VeryLow,
            OpCode::SWAP8 => GasPriceTier::VeryLow,
            OpCode::SWAP9 => GasPriceTier::VeryLow,
            OpCode::SWAP10 => GasPriceTier::VeryLow,
            OpCode::SWAP11 => GasPriceTier::VeryLow,
            OpCode::SWAP12 => GasPriceTier::VeryLow,
            OpCode::SWAP13 => GasPriceTier::VeryLow,
            OpCode::SWAP14 => GasPriceTier::VeryLow,
            OpCode::SWAP15 => GasPriceTier::VeryLow,
            OpCode::SWAP16 => GasPriceTier::VeryLow,
            OpCode::LOG0 => GasPriceTier::Special,
            OpCode::LOG1 => GasPriceTier::Special,
            OpCode::LOG2 => GasPriceTier::Special,
            OpCode::LOG3 => GasPriceTier::Special,
            OpCode::LOG4 => GasPriceTier::Special,
            OpCode::CREATE => GasPriceTier::Special,
            OpCode::CALL => GasPriceTier::Special,
            OpCode::CALLCODE => GasPriceTier::Special,
            OpCode::RETURN => GasPriceTier::Zero,
            OpCode::DELEGATECALL => GasPriceTier::Special,
            OpCode::STATICCALL => GasPriceTier::Special,
            OpCode::SUICIDE => GasPriceTier::Special,
            OpCode::CREATE2 => GasPriceTier::Special,
            OpCode::REVERT => GasPriceTier::Zero,
        }
    }
}
