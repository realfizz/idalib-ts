use napi::bindgen_prelude::BigInt;
use napi_derive::napi;

use crate::database::IdbHandle;
use crate::types::segment::{JsSegment, JsSegmentPermissions};

fn alignment_name(alignment: idalib::segment::SegmentAlignment) -> String {
    use idalib::segment::SegmentAlignment::*;
    match alignment {
        Abs => "Abs",
        RelByte => "RelByte",
        RelWord => "RelWord",
        RelPara => "RelPara",
        RelPage => "RelPage",
        RelDble => "RelDble",
        Rel4K => "Rel4K",
        Group => "Group",
        Rel32Bytes => "Rel32Bytes",
        Rel64Bytes => "Rel64Bytes",
        RelQword => "RelQword",
        Rel128Bytes => "Rel128Bytes",
        Rel512Bytes => "Rel512Bytes",
        Rel1024Bytes => "Rel1024Bytes",
        Rel2048Bytes => "Rel2048Bytes",
    }
    .to_string()
}

fn type_name(seg_type: idalib::segment::SegmentType) -> String {
    use idalib::segment::SegmentType::*;
    match seg_type {
        NORM => "NORM",
        XTRN => "XTRN",
        CODE => "CODE",
        DATA => "DATA",
        IMP => "IMP",
        GRP => "GRP",
        NULL => "NULL",
        UNDF => "UNDF",
        BSS => "BSS",
        ABSSYM => "ABSSYM",
        COMM => "COMM",
        IMEM => "IMEM",
    }
    .to_string()
}

fn snapshot_from_segment(seg: &idalib::segment::Segment) -> JsSegment {
    let permissions: JsSegmentPermissions = seg.permissions().into();
    JsSegment {
        start_address: seg.start_address() as i64,
        end_address: seg.end_address() as i64,
        name: seg.name().unwrap_or_default(),
        permissions,
        alignment: alignment_name(seg.alignment()),
        seg_type: type_name(seg.r#type()),
        color: 0,
        bit_size: 16u32 << seg.bitness(),
    }
}

#[napi]
pub struct SegmentsCollection {
    pub(crate) idb: IdbHandle,
}

#[napi]
impl SegmentsCollection {
    #[napi]
    pub fn list(&self) -> napi::Result<Vec<JsSegment>> {
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        let segments: Vec<JsSegment> = idb
            .0
            .segments()
            .map(|(_, seg)| snapshot_from_segment(&seg))
            .collect();

        Ok(segments)
    }

    #[napi]
    pub fn at(&self, ea: BigInt) -> napi::Result<Option<JsSegment>> {
        let (sign, ea, _) = ea.get_u64();
        if sign {
            return Err(napi::Error::from_reason("Address cannot be negative".to_string()));
        }
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        Ok(idb
            .0
            .segment_at(ea)
            .map(|seg| snapshot_from_segment(&seg)))
    }

    #[napi]
    pub fn by_name(&self, name: String) -> napi::Result<Option<JsSegment>> {
        let guard = self.idb.lock().map_err(|e| {
            napi::Error::from_reason(format!("LockError: {}", e))
        })?;
        let idb = guard
            .as_ref()
            .ok_or_else(|| napi::Error::from_reason("Database is closed".to_string()))?;

        Ok(idb
            .0
            .segment_by_name(&name)
            .map(|seg| snapshot_from_segment(&seg)))
    }
}
