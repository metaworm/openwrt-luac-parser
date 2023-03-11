use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Move = 0,
    LoadK,
    LoadBool,
    LoadNil,
    GetUpval,
    GetGlobal,
    GetTable,
    SetGlobal,
    SetUpval,
    SetTable,
    NewTable,
    Self_,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Unm,
    Not,
    Len,
    Concat,
    Jump,
    Eq,
    Lt,
    Le,
    Test,
    TestSet,
    Call,
    TailCall,
    Return,
    ForLoop,
    ForPrep,
    TForLoop,
    SetList,
    Close,
    Closure,
    VarArg,
}

pub static ORIGIN_OPCODE_MAP: LazyLock<HashMap<String, u8>> = LazyLock::new(|| {
    let mut res = HashMap::new();
    for i in 0..=Opcode::VarArg as u8 {
        let op = unsafe { std::mem::transmute::<_, Opcode>(i) };
        let s = match op {
            Opcode::Self_ => "Self".to_lowercase(),
            Opcode::Jump => "jmp".to_lowercase(),
            _ => format!("{:?}", op).to_lowercase(),
        };
        res.insert(s, i);
    }
    res
});

pub static OPCODE_LIST: &[&str] = &[
    "GETTABLE",
    "GETGLOBAL",
    "SETGLOBAL",
    "SETUPVAL",
    "SETTABLE",
    "NEWTABLE",
    "SELF",
    "LOADNIL",
    "LOADK",
    "LOADBOOL",
    "GETUPVAL",
    "LT",
    "LE",
    "EQ",
    "DIV",
    "MUL",
    "SUB",
    "ADD",
    "MOD",
    "POW",
    "UNM",
    "NOT",
    "LEN",
    "CONCAT",
    "JMP",
    "TEST",
    "TESTSET",
    "MOVE",
    "FORLOOP",
    "FORPREP",
    "TFORLOOP",
    "SETLIST",
    "CLOSE",
    "CLOSURE",
    "CALL",
    "RETURN",
    "TAILCALL",
    "VARARG",
];

pub static OPCODE_MAP: LazyLock<HashMap<u8, String>> = LazyLock::new(|| {
    let mut res = HashMap::new();
    for (i, &op) in OPCODE_LIST.iter().enumerate() {
        res.insert(i as u8, op.to_lowercase());
    }
    res
});
