use crate::game_key::{field::GameKey, serde::SerializableGameKey};
use crate::game_progress::{field::GameProgress, serde::SerializableGameProgress};
use crate::game_record::{field::GameRecord, serde::SerializableGameRecord};
use crate::settings::{field::Settings, serde::SerializableSettings};
use crate::summary::{field::Summary, serde::SerializableSummary};
use crate::user::{field::User, serde::SerializableUser};
use shua_struct::{BinaryField, BitSlice, Lsb0};
use std::alloc::{Layout, alloc, dealloc};
use std::sync::Mutex;

thread_local! {
    static LAST_ERROR: Mutex<String> = Mutex::new(String::new());
}

fn set_error(msg: &str) {
    LAST_ERROR.with(|err| {
        if let Ok(mut e) = err.lock() {
            e.clear();
            e.push_str(msg);
        }
    });
}

#[repr(C)]
pub struct Data {
    pub len: usize,
    pub ptr: *mut u8,
}

#[inline(always)]
pub fn empty_data() -> Data {
    Data {
        len: 0,
        ptr: std::ptr::null_mut(),
    }
}

#[inline(always)]
pub unsafe fn malloc_data(mut bytes: Vec<u8>) -> Data {
    bytes.shrink_to_fit();
    let len = bytes.len();
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    Data { ptr, len }
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(len: usize) -> *mut u8 {
    if len == 0 {
        set_error("无效长度");
        return std::ptr::null_mut();
    }

    unsafe {
        let layout = match Layout::array::<u8>(len) {
            Ok(l) => l,
            Err(e) => {
                set_error(&format!("Layout错误:{}", e));
                return std::ptr::null_mut();
            }
        };

        return alloc(layout);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, len: usize) -> bool {
    unsafe {
        let layout = match Layout::array::<u8>(len) {
            Ok(l) => l,
            Err(e) => {
                set_error(&format!("Layout错误:{}", e));
                return false;
            }
        };
        dealloc(ptr, layout);
        return true;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_last_error() -> Data {
    LAST_ERROR.with(|err| {
        if let Ok(e) = err.lock() {
            if e.is_empty() {
                return empty_data();
            }
            let error_bytes = e.as_bytes().to_vec();
            unsafe { malloc_data(error_bytes) }
        } else {
            empty_data()
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn clear_last_error() -> bool {
    LAST_ERROR.with(|err| match err.lock() {
        Ok(mut e) => {
            e.clear();
            true
        }
        Err(_) => false,
    })
}

#[macro_export]
macro_rules! impl_c_api {
    ($struct_ty:ty, $serializable_ty:ty, $parse_fn:ident, $build_fn:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $parse_fn(data_ptr: *const u8, data_len: usize) -> Data {
            if data_ptr.is_null() || data_len == 0 {
                set_error("空或无效的输入数据");
                return empty_data();
            }
            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let bits = BitSlice::<u8, Lsb0>::from_slice(bytes);

            let (item, _) = match <$struct_ty>::parse(bits, &None) {
                Ok(r) => r,
                Err(e) => {
                    set_error(&format!("解析错误: {}", e));
                    return empty_data();
                }
            };

            let json = match rmp_serde::to_vec_named(&<$serializable_ty>::from(item)) {
                Ok(v) => v,
                Err(e) => {
                    set_error(&format!("序列化错误: {}", e));
                    return empty_data();
                }
            };

            unsafe { malloc_data(json) }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $build_fn(data_ptr: *const u8, data_len: usize) -> Data {
            if data_ptr.is_null() || data_len == 0 {
                set_error("空或无效的输入数据");
                return empty_data();
            }

            let bytes = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
            let serializable: $serializable_ty = match rmp_serde::from_slice(bytes) {
                Ok(v) => v,
                Err(e) => {
                    set_error(&format!("反序列化错误: {}", e));
                    return empty_data();
                }
            };

            let bitvec = match <$struct_ty>::from(serializable).build(&None) {
                Ok(v) => v,
                Err(e) => {
                    set_error(&format!("构建错误: {}", e));
                    return empty_data();
                }
            };

            unsafe { malloc_data(bitvec.into_vec()) }
        }
    };
}

impl_c_api!(User, SerializableUser, parse_user, build_user);
impl_c_api!(Summary, SerializableSummary, parse_summary, build_summary);
impl_c_api!(
    GameRecord,
    SerializableGameRecord,
    parse_game_record,
    build_game_record
);
impl_c_api!(
    GameProgress,
    SerializableGameProgress,
    parse_game_progress,
    build_game_progress
);
impl_c_api!(GameKey, SerializableGameKey, parse_game_key, build_game_key);
impl_c_api!(
    Settings,
    SerializableSettings,
    parse_settings,
    build_settings
);
