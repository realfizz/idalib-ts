use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

use crate::database::IdbHandle;

#[napi]
pub struct BytesAccess {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl BytesAccess {
    fn with_idb<T>(&self, f: impl FnOnce(&idalib::IDB) -> T) -> napi::Result<T> {
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;
        Ok(f(&idb.0))
    }

    #[napi]
    pub fn get_byte(&self, ea: BigInt) -> napi::Result<u8> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.get_byte(ea))
    }

    #[napi]
    pub fn get_word(&self, ea: BigInt) -> napi::Result<u16> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.get_word(ea))
    }

    #[napi]
    pub fn get_dword(&self, ea: BigInt) -> napi::Result<u32> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.get_dword(ea))
    }

    #[napi]
    pub fn get_qword(&self, ea: BigInt) -> napi::Result<u64> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.get_qword(ea))
    }

    #[napi]
    pub fn read(&self, ea: BigInt, size: u32) -> napi::Result<Vec<u8>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        self.with_idb(|idb| idb.get_bytes(ea, size as usize))
    }
}
