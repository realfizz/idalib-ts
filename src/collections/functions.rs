use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

use crate::database::IdbHandle;
use crate::types::function::JsFunction;

#[napi(object)]
pub struct AddressRange {
    pub start: i64,
    pub end: i64,
}

#[napi(object)]
pub struct FunctionQuery {
    pub prefix: Option<String>,
    pub address_range: Option<AddressRange>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[napi]
pub struct FunctionsCollection {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl FunctionsCollection {
    #[napi]
    pub async fn list(&self) -> napi::Result<Vec<JsFunction>> {
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        let mut functions = Vec::new();
        for (_, func) in idb.0.functions() {
            functions.push(JsFunction::from_func(&func));
        }

        Ok(functions)
    }

    #[napi]
    pub async fn at(&self, ea: BigInt) -> napi::Result<Option<JsFunction>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        Ok(idb.0.function_at(ea).map(|f| JsFunction::from_func(&f)))
    }

    #[napi]
    pub async fn by_id(&self, id: u32) -> napi::Result<Option<JsFunction>> {
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        Ok(idb.0.function_by_id(id as usize).map(|f| JsFunction::from_func(&f)))
    }

    #[napi]
    pub async fn count(&self) -> napi::Result<u32> {
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        Ok(idb.0.function_count() as u32)
    }

    #[napi]
    pub async fn query(&self, options: FunctionQuery) -> napi::Result<Vec<JsFunction>> {
        let guard = self
            .idb
            .lock()
            .map_err(|e| napi::Error::from_reason(format!("LockError: {}", e)))?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        let mut results = Vec::new();
        let offset = options.offset.unwrap_or(0) as usize;
        let limit = options.limit.map(|l| l as usize);
        let mut skipped = 0usize;

        for (_, func) in idb.0.functions() {
            if let Some(ref prefix) = options.prefix {
                if !func.name().map_or(true, |n| n.starts_with(prefix)) {
                    continue;
                }
            }

            if let Some(ref range) = options.address_range {
                let ea = func.start_address() as i64;
                if ea < range.start || ea >= range.end {
                    continue;
                }
            }

            if skipped < offset {
                skipped += 1;
                continue;
            }

            if let Some(limit) = limit {
                if results.len() >= limit {
                    break;
                }
            }

            results.push(JsFunction::from_func(&func));
        }

        Ok(results)
    }
}
