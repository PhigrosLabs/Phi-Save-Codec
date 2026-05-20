use phi_save_codec::Binary;
use phi_save_codec::GameKey;
use phi_save_codec::GameProgress;
use phi_save_codec::GameRecord;
use phi_save_codec::Settings;
use phi_save_codec::Summary;
use phi_save_codec::User;
use std::alloc::{Layout, alloc, dealloc};

#[repr(C)]
pub struct Data {
    pub tag: usize,
    pub len: usize,
    pub ptr: *mut u8,
}

impl Data {
    fn ok(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        bytes.shrink_to_fit();
        let len = bytes.len();
        let ptr = bytes.as_mut_ptr();
        std::mem::forget(bytes);
        Data { tag: 0, len, ptr }
    }

    fn err(msg: &str) -> Self {
        let mut bytes = msg.as_bytes().to_vec();
        bytes.shrink_to_fit();
        let len = bytes.len();
        let ptr = bytes.as_mut_ptr();
        std::mem::forget(bytes);
        Data { tag: 1, len, ptr }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn psc_malloc(len: usize) -> *mut u8 {
    if len == 0 {
        return std::ptr::null_mut();
    }
    unsafe {
        let Ok(layout) = Layout::array::<u8>(len) else {
            return std::ptr::null_mut();
        };
        alloc(layout)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn psc_free(ptr: *mut u8, len: usize) -> bool {
    unsafe {
        let Ok(layout) = Layout::array::<u8>(len) else {
            return false;
        };
        dealloc(ptr, layout);
        true
    }
}

#[macro_export]
macro_rules! impl_c_api {
    ($ty:ty, $parse_fn:ident, $build_fn:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $parse_fn(data_ptr: *const u8, data_len: usize) -> $crate::Data {
            if data_ptr.is_null() || data_len == 0 {
                return $crate::Data::err("空或无效的输入数据");
            }
            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let item = match <$ty>::read(bytes) {
                Ok(r) => r,
                Err(e) => return $crate::Data::err(&format!("解析错误: {:?}", e)),
            };
            match rmp_serde::to_vec_named(&item) {
                Ok(v) => $crate::Data::ok(v),
                Err(e) => $crate::Data::err(&format!("序列化错误: {:?}", e)),
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $build_fn(data_ptr: *const u8, data_len: usize) -> $crate::Data {
            if data_ptr.is_null() || data_len == 0 {
                return $crate::Data::err("空或无效的输入数据");
            }
            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let item: $ty = match rmp_serde::from_slice(bytes) {
                Ok(v) => v,
                Err(e) => return $crate::Data::err(&format!("反序列化错误: {:?}", e)),
            };
            let buf_len = item.len();
            let mut buf = vec![0u8; buf_len];
            if let Err(e) = item.write(&mut buf) {
                return $crate::Data::err(&format!("构建错误: {:?}", e));
            }
            $crate::Data::ok(buf)
        }
    };
}

impl_c_api!(User, psc_parse_user, psc_build_user);
impl_c_api!(Summary, psc_parse_summary, psc_build_summary);
impl_c_api!(GameRecord, psc_parse_game_record, psc_build_game_record);
impl_c_api!(
    GameProgress,
    psc_parse_game_progress,
    psc_build_game_progress
);
impl_c_api!(GameKey, psc_parse_game_key, psc_build_game_key);
impl_c_api!(Settings, psc_parse_settings, psc_build_settings);
