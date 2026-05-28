use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

use crate::database::IdbHandle;
use crate::types::xref::JsXRef;

#[napi(object)]
pub struct XRefQueryOpt {
    pub type_: Option<String>,
}

fn xref_query_from_opts(opts: Option<&XRefQueryOpt>) -> idalib::xref::XRefQuery {
    match opts.and_then(|o| o.type_.as_deref()) {
        Some("data") => idalib::xref::XRefQuery::DATA,
        _ => idalib::xref::XRefQuery::ALL,
    }
}

#[napi]
pub struct XRefsCollection {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl XRefsCollection {
    #[napi]
    pub fn to(&self, ea: BigInt, query: Option<XRefQueryOpt>) -> napi::Result<Option<JsXRef>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard.as_ref().ok_or_else(|| {
            napi::Error::from_reason("Database is closed".to_string())
        })?;

        let query_type = query.as_ref().and_then(|q| q.type_.as_deref());
        let idalib_query = xref_query_from_opts(query.as_ref());

        let mut xref = match idb.0.first_xref_to(ea, idalib_query) {
            Some(x) => x,
            None => return Ok(None),
        };

        if query_type == Some("code") {
            while !xref.is_code() {
                match xref.next_to() {
                    Some(x) => xref = x,
                    None => return Ok(None),
                }
            }
        }

        Ok(Some(JsXRef::new(&xref)))
    }

    #[napi]
    pub fn from(&self, ea: BigInt, query: Option<XRefQueryOpt>) -> napi::Result<Option<JsXRef>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard.as_ref().ok_or_else(|| {
            napi::Error::from_reason("Database is closed".to_string())
        })?;

        let query_type = query.as_ref().and_then(|q| q.type_.as_deref());
        let idalib_query = xref_query_from_opts(query.as_ref());

        let mut xref = match idb.0.first_xref_from(ea, idalib_query) {
            Some(x) => x,
            None => return Ok(None),
        };

        if query_type == Some("code") {
            while !xref.is_code() {
                match xref.next_from() {
                    Some(x) => xref = x,
                    None => return Ok(None),
                }
            }
        }

        Ok(Some(JsXRef::new(&xref)))
    }
}
