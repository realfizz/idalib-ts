use napi_derive::napi;

mod collections;
mod comment;
mod database;
mod error;
mod types;

fn ensure_init() {
    idalib::init_library();
}

#[napi]
pub fn force_batch_mode() {
    ensure_init();
    idalib::force_batch_mode();
}

#[napi]
pub fn enable_console_messages(enabled: bool) {
    ensure_init();
    idalib::enable_console_messages(enabled);
}

#[napi]
pub async fn version() -> napi::Result<String> {
    ensure_init();
    let v = idalib::version().map_err(crate::error::map_ida_error)?;
    Ok(format!("{}.{}.{}", v.major(), v.minor(), v.build()))
}

#[napi]
pub async fn is_license_valid() -> bool {
    ensure_init();
    idalib::is_valid_license()
}

#[napi]
pub async fn license_id() -> napi::Result<String> {
    ensure_init();
    let id = idalib::license_id().map_err(crate::error::map_ida_error)?;
    Ok(id.to_string())
}

#[napi]
pub async fn open(path: String, options: Option<database::OpenOptions>) -> napi::Result<database::Database> {
    database::Database::open(path, options).await
}

pub use database::Database;
pub use error::{IDALibError, DatabaseOpenError, LicenseError, DecompilerError, PluginError};
