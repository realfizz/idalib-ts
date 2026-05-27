use std::sync::{Arc, Mutex};

use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

use crate::collections::{
    bookmarks::BookmarksCollection, bytes::BytesAccess, functions::FunctionsCollection,
    names::NamesCollection, segments::SegmentsCollection, strings::StringsCollection,
    xrefs::XRefsCollection,
};
use crate::comment;
use crate::error::map_ida_error;
use crate::types::decompiler::JsCFunction;
use crate::types::function::JsFunction;
use crate::types::instruction::Instruction;
use crate::types::metadata::{JsEntryPoint, JsMetadata};
use crate::types::plugin::JsPlugin;

pub(crate) struct SendIdb(pub(crate) idalib::IDB);

unsafe impl Send for SendIdb {}

pub(crate) type IdbHandle = Arc<Mutex<Option<SendIdb>>>;

#[napi]
pub struct Database {
    pub(crate) idb: IdbHandle,
}

#[napi(object)]
pub struct OpenOptions {
    pub ida_dir: Option<String>,
    pub auto_analyze: Option<bool>,
    pub save: Option<bool>,
    pub file_type: Option<String>,
    pub idb_path: Option<String>,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            ida_dir: None,
            auto_analyze: None,
            save: None,
            file_type: None,
            idb_path: None,
        }
    }
}

#[napi]
impl Database {
    fn lock_guard(&self) -> napi::Result<std::sync::MutexGuard<'_, Option<SendIdb>>> {
        self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })
    }

    fn with_idb<T: 'static>(&self, f: impl FnOnce(&idalib::IDB) -> T) -> napi::Result<T> {
        let guard = self.lock_guard()?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;
        Ok(f(&idb.0))
    }

    fn with_idb_mut<T: 'static>(&self, f: impl FnOnce(&mut idalib::IDB) -> T) -> napi::Result<T> {
        let mut guard = self.lock_guard()?;
        let idb = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;
        Ok(f(&mut idb.0))
    }

    fn try_with_idb<T: 'static>(&self, f: impl FnOnce(&idalib::IDB) -> napi::Result<T>) -> napi::Result<T> {
        let guard = self.lock_guard()?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;
        f(&idb.0)
    }

    fn try_with_idb_mut<T: 'static>(&self, f: impl FnOnce(&mut idalib::IDB) -> napi::Result<T>) -> napi::Result<T> {
        let mut guard = self.lock_guard()?;
        let idb = guard
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;
        f(&mut idb.0)
    }

    #[napi]
    pub async fn open(path: String, options: Option<OpenOptions>) -> napi::Result<Self> {
        let opts = options.unwrap_or_default();
        let auto_analyse = opts.auto_analyze.unwrap_or(true);
        let save = opts.save.unwrap_or(false);
        let needs_custom_options = opts.file_type.is_some() || opts.idb_path.is_some();

        if let Some(ref dir) = opts.ida_dir {
            std::env::set_var("IDADIR", dir);
        }

        let idb = if needs_custom_options {
            let mut builder = idalib::IDBOpenOptions::new();
            builder.auto_analyse(auto_analyse);
            builder.save(save);
            if let Some(ft) = &opts.file_type {
                builder.file_type(ft);
            }
            if let Some(idb_path) = &opts.idb_path {
                builder.idb(idb_path);
            }
            builder.open(&path).map_err(map_ida_error)?
        } else {
            idalib::IDB::open_with(&path, auto_analyse, save).map_err(map_ida_error)?
        };

        Ok(Database {
            idb: Arc::new(Mutex::new(Some(SendIdb(idb)))),
        })
    }

    #[napi]
    pub async fn close(&self) -> napi::Result<()> {
        let mut guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        *guard = None;
        Ok(())
    }

    #[napi(getter)]
    pub async fn path(&self) -> napi::Result<String> {
        self.with_idb(|idb| idb.path().to_string_lossy().to_string())
    }

    #[napi(getter)]
    pub fn functions(&self) -> FunctionsCollection {
        FunctionsCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn segments(&self) -> SegmentsCollection {
        SegmentsCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn strings(&self) -> StringsCollection {
        StringsCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn names(&self) -> NamesCollection {
        NamesCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn bookmarks(&self) -> BookmarksCollection {
        BookmarksCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn xrefs(&self) -> XRefsCollection {
        XRefsCollection {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn bytes(&self) -> BytesAccess {
        BytesAccess {
            idb: self.idb.clone(),
        }
    }

    #[napi(getter)]
    pub fn decompiler_available(&self) -> napi::Result<bool> {
        self.with_idb(|idb| idb.decompiler_available())
    }

    #[napi]
    pub async fn metadata(&self) -> napi::Result<JsMetadata> {
        self.with_idb(|idb| {
            let meta = idb.meta();
            JsMetadata {
                abi: meta.procname(),
                abi_features: format!("{:#x}", meta.abibits()),
                analysis_flags: meta.af().bits(),
                compiler: format!("{:?}", meta.cc_id()),
                file_type: format!("{:?}", meta.filetype()),
                image_base: meta.base_address().unwrap_or(0) as i64,
            }
        })
    }

    #[napi]
    pub async fn address_to_string(&self, ea: BigInt) -> napi::Result<Option<String>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.address_to_string(ea))
    }

    #[napi]
    pub async fn register_by_name(&self, name: String) -> napi::Result<Option<u16>> {
        self.with_idb(|idb| idb.register_by_name(name))
    }

    #[napi]
    pub async fn entries(&self) -> napi::Result<Vec<JsEntryPoint>> {
        self.with_idb(|idb| {
            let mut result = Vec::new();
            for (i, addr) in idb.entries().enumerate() {
                let address: u64 = addr;
                result.push(JsEntryPoint {
                    ordinal: i as u32,
                    address: address as i64,
                });
            }
            result
        })
    }

    #[napi]
    pub async fn set_screen_address(&self, ea: BigInt) -> napi::Result<()> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb_mut(|idb| idb.set_screen_address(ea))
    }

    #[napi]
    pub async fn save_on_close(&self, status: bool) -> napi::Result<()> {
        self.with_idb_mut(|idb| idb.save_on_close(status))
    }

    #[napi]
    pub async fn make_signatures(&self, only_pat: Option<bool>) -> napi::Result<()> {
        let only_pat = only_pat.unwrap_or(false);
        self.try_with_idb_mut(|idb| idb.make_signatures(only_pat).map_err(map_ida_error))
    }

    #[napi]
    pub async fn load_plugin(&self, name: String) -> napi::Result<JsPlugin> {
        self.try_with_idb(|idb| {
            idb.load_plugin(&name).map_err(map_ida_error)?;
            Ok(JsPlugin { name })
        })
    }

    #[napi]
    pub async fn find_plugin(
        &self,
        name: String,
        load_if_needed: bool,
    ) -> napi::Result<JsPlugin> {
        self.try_with_idb(|idb| {
            idb
                .find_plugin(&name, load_if_needed)
                .map_err(map_ida_error)?;
            Ok(JsPlugin { name })
        })
    }

    #[napi]
    pub async fn auto_wait(&self) -> napi::Result<bool> {
        self.with_idb_mut(|idb| idb.auto_wait())
    }

    #[napi]
    pub async fn function_cfg(&self, func: &JsFunction) -> napi::Result<crate::types::function::JsFunctionCfg> {
        self.try_with_idb(|idb| {
            let f = idb
                .function_at(func.start_ea as u64)
                .ok_or_else(|| napi::Error::from_reason("Function not found".to_string()))?;
            let cfg = f.cfg().map_err(|e| {
                napi::Error::from_reason(format!("CFGError: {}", e))
            })?;
            Ok(crate::types::function::JsFunctionCfg::from_cfg(&cfg))
        })
    }

    #[napi]
    pub async fn instruction_at(&self, ea: BigInt) -> napi::Result<Option<Instruction>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        let insn = self.with_idb(|idb| idb.insn_at(ea))?;
        Ok(insn.map(|i| Instruction { inner: i }))
    }

    #[napi]
    pub async fn next_head(&self, ea: BigInt, max_ea: Option<BigInt>) -> napi::Result<Option<u64>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| {
            match max_ea {
                Some(max) => {
                    let (sign, max, _) = max.get_u64();
                    if sign {
                        return Err(napi::Error::from_reason("Bound address cannot be negative".to_string()));
                    }
                    Ok(idb.next_head_with(ea, max))
                }
                None => Ok(idb.next_head(ea)),
            }
        })
    }

    #[napi]
    pub async fn prev_head(&self, ea: BigInt, min_ea: Option<BigInt>) -> napi::Result<Option<u64>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| {
            match min_ea {
                Some(min) => {
                    let (sign, min, _) = min.get_u64();
                    if sign {
                        return Err(napi::Error::from_reason("Bound address cannot be negative".to_string()));
                    }
                    Ok(idb.prev_head_with(ea, min))
                }
                None => Ok(idb.prev_head(ea)),
            }
        })
    }

    #[napi]
    pub async fn instruction_alignment_at(&self, ea: BigInt) -> napi::Result<Option<u32>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.insn_alignment_at(ea).map(|a| a as u32))
    }

    #[napi]
    pub async fn find_text(&self, start_ea: BigInt, text: String) -> napi::Result<Option<u64>> {
        let (sign, ea, _) = start_ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.find_text(ea, &text))
    }

    #[napi]
    pub async fn find_imm(&self, start_ea: BigInt, value: u32) -> napi::Result<Option<u64>> {
        let (sign, ea, _) = start_ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.find_imm(ea, value))
    }

    #[napi]
    pub async fn find_defined(&self, start_ea: BigInt) -> napi::Result<Option<u64>> {
        let (sign, ea, _) = start_ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.find_defined(ea))
    }

    #[napi]
    pub async fn get_comment(&self, ea: BigInt, repeatable: Option<bool>) -> napi::Result<Option<String>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| comment::get_comment(idb, ea, repeatable))
    }

    #[napi]
    pub async fn set_comment(
        &self,
        ea: BigInt,
        comment: String,
        repeatable: Option<bool>,
    ) -> napi::Result<()> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| comment::set_comment(idb, ea_val, &comment, repeatable))
    }

    #[napi]
    pub async fn append_comment(
        &self,
        ea: BigInt,
        comment: String,
        repeatable: Option<bool>,
    ) -> napi::Result<()> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| comment::append_comment(idb, ea_val, &comment, repeatable))
    }

    #[napi]
    pub async fn remove_comment(&self, ea: BigInt, repeatable: Option<bool>) -> napi::Result<()> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| comment::remove_comment(idb, ea_val, repeatable))
    }

    #[napi]
    pub async fn get_function_comment(
        &self,
        ea: BigInt,
        repeatable: Option<bool>,
    ) -> napi::Result<Option<String>> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| comment::get_function_comment(idb, ea_val, repeatable))
    }

    #[napi]
    pub async fn set_function_comment(
        &self,
        ea: BigInt,
        comment: String,
        repeatable: Option<bool>,
    ) -> napi::Result<()> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| comment::set_function_comment(idb, ea_val, &comment, repeatable))
    }

    #[napi]
    pub async fn remove_function_comment(&self, ea: BigInt, repeatable: Option<bool>) -> napi::Result<()> {
        let (sign, ea_val, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.try_with_idb(|idb| comment::remove_function_comment(idb, ea_val, repeatable))
    }

    #[napi]
    pub async fn decompile(
        &self,
        func: &JsFunction,
        options: Option<DecompileOptions>,
    ) -> napi::Result<JsCFunction> {
        let all_blocks_flag = options.map(|o| o.all_blocks).unwrap_or(true);

        let available = self.decompiler_available()?;
        if !available {
            return Err(napi::Error::from_reason(
                "DecompilerError: Decompiler is not available".to_string(),
            ));
        }

        let ea = func.get_start_ea();

        self.try_with_idb(|_idb| {
            let func_ptr = unsafe { idalib::ffi::func::get_func(ea.into()) };
            if func_ptr.is_null() {
                return Err(napi::Error::from_reason(
                    "DecompilerError: No function at address".to_string(),
                ));
            }
            let cfunc_ptr = unsafe {
                idalib::ffi::hexrays::decompile_func(func_ptr, all_blocks_flag)
            }
            .map_err(|e| {
                napi::Error::from_reason(format!("DecompilerError: {}", e.reason()))
            })?;
            Ok(crate::types::decompiler::snapshot_cfunc(&cfunc_ptr))
        })
    }
}

#[napi(object)]
pub struct DecompileOptions {
    pub all_blocks: bool,
}
