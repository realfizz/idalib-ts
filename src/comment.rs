use crate::error::map_ida_error;
use idalib::Address;

fn to_addr(ea: u64) -> Address {
    Address::from(ea)
}

pub(crate) fn get_comment(idb: &idalib::IDB, ea: u64, repeatable: Option<bool>) -> Option<String> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb.get_cmt_with(addr, true),
        _ => idb.get_cmt(addr),
    }
}

pub(crate) fn set_comment(
    idb: &idalib::IDB,
    ea: u64,
    comment: &str,
    repeatable: Option<bool>,
) -> napi::Result<()> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb.set_cmt_with(addr, comment, true).map_err(map_ida_error),
        _ => idb.set_cmt(addr, comment).map_err(map_ida_error),
    }
}

pub(crate) fn append_comment(
    idb: &idalib::IDB,
    ea: u64,
    comment: &str,
    repeatable: Option<bool>,
) -> napi::Result<()> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb
            .append_cmt_with(addr, comment, true)
            .map_err(map_ida_error),
        _ => idb.append_cmt(addr, comment).map_err(map_ida_error),
    }
}

pub(crate) fn remove_comment(
    idb: &idalib::IDB,
    ea: u64,
    repeatable: Option<bool>,
) -> napi::Result<()> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb.remove_cmt_with(addr, true).map_err(map_ida_error),
        _ => idb.remove_cmt(addr).map_err(map_ida_error),
    }
}

pub(crate) fn get_function_comment(
    idb: &idalib::IDB,
    ea: u64,
    repeatable: Option<bool>,
) -> Option<String> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb.get_func_cmt_with(addr, true),
        _ => idb.get_func_cmt(addr),
    }
}

pub(crate) fn set_function_comment(
    idb: &idalib::IDB,
    ea: u64,
    comment: &str,
    repeatable: Option<bool>,
) -> napi::Result<()> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb
            .set_func_cmt_with(addr, comment, true)
            .map_err(map_ida_error),
        _ => idb.set_func_cmt(addr, comment).map_err(map_ida_error),
    }
}

pub(crate) fn remove_function_comment(
    idb: &idalib::IDB,
    ea: u64,
    repeatable: Option<bool>,
) -> napi::Result<()> {
    let addr = to_addr(ea);
    match repeatable {
        Some(true) => idb
            .remove_func_cmt_with(addr, true)
            .map_err(map_ida_error),
        _ => idb.remove_func_cmt(addr).map_err(map_ida_error),
    }
}
