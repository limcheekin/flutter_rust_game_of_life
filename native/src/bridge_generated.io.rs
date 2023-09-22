use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_connect__method__SurrealDB(
    port_: i64,
    that: *mut wire_SurrealDB,
    endpoint: *mut wire_uint_8_list,
) {
    wire_connect__method__SurrealDB_impl(port_, that, endpoint)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_SurrealAny() -> wire_SurrealAny {
    wire_SurrealAny::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_surreal_db_0() -> *mut wire_SurrealDB {
    support::new_leak_box_ptr(wire_SurrealDB::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<RustOpaque<Surreal<Any>>> for wire_SurrealAny {
    fn wire2api(self) -> RustOpaque<Surreal<Any>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<SurrealDB> for *mut wire_SurrealDB {
    fn wire2api(self) -> SurrealDB {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SurrealDB>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SurrealDB> for wire_SurrealDB {
    fn wire2api(self) -> SurrealDB {
        SurrealDB {
            db: self.db.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_SurrealAny {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SurrealDB {
    db: wire_SurrealAny,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_SurrealAny {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_SurrealDB {
    fn new_with_null_ptr() -> Self {
        Self {
            db: wire_SurrealAny::new_with_null_ptr(),
        }
    }
}

impl Default for wire_SurrealDB {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
