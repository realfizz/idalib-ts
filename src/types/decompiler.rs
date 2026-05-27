use napi_derive::napi;

#[napi(object)]
pub struct JsCFunction {
    pub entry_point: i64,
    pub blocks: Vec<JsCBlock>,
    #[napi(js_name = "pseudoCode")]
    pub pseudo_code: String,
    pub error: Option<String>,
}

#[napi(object)]
pub struct JsCBlock {
    pub start_address: i64,
    pub end_address: i64,
    pub r#type: u32,
    pub instructions: Vec<JsCInsn>,
}

#[napi(object)]
pub struct JsCInsn {
    pub address: i64,
    pub operation: String,
    pub operands: Vec<String>,
    pub text: String,
}

pub(crate) fn snapshot_cfunc(
    cfunc_ref: &idalib::ffi::hexrays::cfuncptr_t,
) -> JsCFunction {
    let cfunc_inner = unsafe {
        idalib::ffi::hexrays::idalib_hexrays_cfuncptr_inner(
            cfunc_ref as *const _ as *const _,
        )
    };
    if cfunc_inner.is_null() {
        return JsCFunction {
            entry_point: 0,
            blocks: Vec::new(),
            pseudo_code: String::new(),
            error: Some("Null cfunc_inner from cfuncptr_t".to_string()),
        };
    }
    let cf = unsafe { &*cfunc_inner };
    let entry_point = cf.entry_ea as i64;
    let pseudo_code =
        unsafe { idalib::ffi::hexrays::idalib_hexrays_cfunc_pseudocode(cfunc_inner) };
    let blocks = collect_blocks(&cf.body);
    JsCFunction {
        entry_point,
        blocks,
        pseudo_code,
        error: None,
    }
}

const CIT_BLOCK: u32 = 71;
const MAX_BLOCK_DEPTH: u32 = 256;

fn collect_blocks(body: &idalib::ffi::hexrays::cinsn_t) -> Vec<JsCBlock> {
    let mut blocks = Vec::new();
    collect_blocks_inner(body, &mut blocks, 0);
    blocks
}

fn collect_blocks_inner(
    cinsn: &idalib::ffi::hexrays::cinsn_t,
    blocks: &mut Vec<JsCBlock>,
    depth: u32,
) {
    if depth >= MAX_BLOCK_DEPTH {
        return;
    }
    if cinsn._base.op != CIT_BLOCK {
        return;
    }
    let cblock_ptr = unsafe { cinsn.__bindgen_anon_1.cblock };
    if cblock_ptr.is_null() {
        return;
    }
    let mut instructions: Vec<JsCInsn> = Vec::new();
    let mut min_ea = u64::MAX;
    let mut max_ea = u64::MIN;
    let mut iter = unsafe { idalib::ffi::hexrays::idalib_hexrays_cblock_iter(cblock_ptr) };
    loop {
        let child_ptr = unsafe {
            idalib::ffi::hexrays::idalib_hexrays_cblock_iter_next(iter.pin_mut())
        };
        if child_ptr.is_null() {
            break;
        }
        let child = unsafe { &*child_ptr };
        let ea = child._base.ea;
        let badaddr = idalib::ffi::BADADDR.0;
        if ea != badaddr {
            if ea < min_ea {
                min_ea = ea;
            }
            if ea > max_ea {
                max_ea = ea;
            }
        }
        if child._base.op == CIT_BLOCK {
            collect_blocks_inner(child, blocks, depth + 1);
        }
        instructions.push(snapshot_insn(child));
    }
    if min_ea != u64::MAX {
        blocks.push(JsCBlock {
            start_address: min_ea as i64,
            end_address: max_ea as i64,
            r#type: CIT_BLOCK,
            instructions,
        });
    }
}

fn snapshot_insn(insn: &idalib::ffi::hexrays::cinsn_t) -> JsCInsn {
    let operation = match insn._base.op {
        0..=69 => format!("expr({})", insn._base.op),
        70 => "empty".to_string(),
        CIT_BLOCK => "block".to_string(),
        72 => "expression".to_string(),
        73 => "if".to_string(),
        74 => "for".to_string(),
        75 => "while".to_string(),
        76 => "do".to_string(),
        77 => "switch".to_string(),
        78 => "break".to_string(),
        79 => "continue".to_string(),
        80 => "return".to_string(),
        81 => "goto".to_string(),
        82 => "asm".to_string(),
        83 => "try".to_string(),
        84 => "throw".to_string(),
        _ => format!("unknown({})", insn._base.op),
    };
    JsCInsn {
        address: insn._base.ea as i64,
        text: operation.clone(),
        operation,
        operands: Vec::new(),
    }
}
