use napi_derive::napi;

#[napi]
pub struct IDALibError;

#[napi]
pub struct DatabaseOpenError;

#[napi]
pub struct LicenseError;

#[napi]
pub struct DecompilerError;

#[napi]
pub struct PluginError;

pub fn map_ida_error(e: idalib::IDAError) -> napi::Error {
    match e {
        idalib::IDAError::Init(_) => {
            napi::Error::from_reason(format!("IDALibError: {e}"))
        }
        idalib::IDAError::FileNotFound { .. } => {
            napi::Error::from_reason(format!("IDALibError: {e}"))
        }
        idalib::IDAError::OpenDb { .. } => {
            napi::Error::from_reason(format!("DatabaseOpenError: {e}"))
        }
        idalib::IDAError::CloseDb(_) => {
            napi::Error::from_reason(format!("IDALibError: {e}"))
        }
        idalib::IDAError::InvalidLicense => {
            napi::Error::from_reason("LicenseError: Invalid idalib license".to_string())
        }
        idalib::IDAError::Ffi(_) => {
            napi::Error::from_reason(format!("IDALibError: {e}"))
        }
        idalib::IDAError::HexRays(_) => {
            napi::Error::from_reason(format!("DecompilerError: {e}"))
        }
        idalib::IDAError::MakeSigs => {
            napi::Error::from_reason(format!("IDALibError: {e}"))
        }
        idalib::IDAError::GetVersion => {
            napi::Error::from_reason("IDALibError: Failed to get version".to_string())
        }
    }
}
