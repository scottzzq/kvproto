// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct LockInfo {
    // message fields
    primary_lock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock_version: ::std::option::Option<u64>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock_ttl: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LockInfo {}

impl LockInfo {
    pub fn new() -> LockInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LockInfo {
        static mut instance: ::protobuf::lazy::Lazy<LockInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockInfo,
        };
        unsafe {
            instance.get(LockInfo::new)
        }
    }

    // optional bytes primary_lock = 1;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    pub fn has_primary_lock(&self) -> bool {
        self.primary_lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary_lock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.primary_lock.is_none() {
            self.primary_lock.set_default();
        };
        self.primary_lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::std::vec::Vec<u8> {
        self.primary_lock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary_lock(&self) -> &[u8] {
        match self.primary_lock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_primary_lock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.primary_lock
    }

    fn mut_primary_lock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.primary_lock
    }

    // optional uint64 lock_version = 2;

    pub fn clear_lock_version(&mut self) {
        self.lock_version = ::std::option::Option::None;
    }

    pub fn has_lock_version(&self) -> bool {
        self.lock_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = ::std::option::Option::Some(v);
    }

    pub fn get_lock_version(&self) -> u64 {
        self.lock_version.unwrap_or(0)
    }

    fn get_lock_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lock_version
    }

    fn mut_lock_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lock_version
    }

    // optional bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint64 lock_ttl = 4;

    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = ::std::option::Option::None;
    }

    pub fn has_lock_ttl(&self) -> bool {
        self.lock_ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = ::std::option::Option::Some(v);
    }

    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl.unwrap_or(0)
    }

    fn get_lock_ttl_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lock_ttl
    }

    fn mut_lock_ttl_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lock_ttl
    }
}

impl ::protobuf::Message for LockInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary_lock)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.lock_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.lock_ttl = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.primary_lock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.lock_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.lock_ttl {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary_lock.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.lock_version {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.lock_ttl {
            os.write_uint64(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LockInfo {
    fn new() -> LockInfo {
        LockInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<LockInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "primary_lock",
                    LockInfo::get_primary_lock_for_reflect,
                    LockInfo::mut_primary_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_version",
                    LockInfo::get_lock_version_for_reflect,
                    LockInfo::mut_lock_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    LockInfo::get_key_for_reflect,
                    LockInfo::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_ttl",
                    LockInfo::get_lock_ttl_for_reflect,
                    LockInfo::mut_lock_ttl_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockInfo>(
                    "LockInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LockInfo {
    fn clear(&mut self) {
        self.clear_primary_lock();
        self.clear_lock_version();
        self.clear_key();
        self.clear_lock_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyError {
    // message fields
    locked: ::protobuf::SingularPtrField<LockInfo>,
    retryable: ::protobuf::SingularField<::std::string::String>,
    abort: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyError {}

impl KeyError {
    pub fn new() -> KeyError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyError {
        static mut instance: ::protobuf::lazy::Lazy<KeyError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyError,
        };
        unsafe {
            instance.get(KeyError::new)
        }
    }

    // optional .kvrpcpb.LockInfo locked = 1;

    pub fn clear_locked(&mut self) {
        self.locked.clear();
    }

    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked.set_default();
        };
        self.locked.as_mut().unwrap()
    }

    // Take field
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(|| LockInfo::new())
    }

    pub fn get_locked(&self) -> &LockInfo {
        self.locked.as_ref().unwrap_or_else(|| LockInfo::default_instance())
    }

    fn get_locked_for_reflect(&self) -> &::protobuf::SingularPtrField<LockInfo> {
        &self.locked
    }

    fn mut_locked_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LockInfo> {
        &mut self.locked
    }

    // optional string retryable = 2;

    pub fn clear_retryable(&mut self) {
        self.retryable.clear();
    }

    pub fn has_retryable(&self) -> bool {
        self.retryable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryable(&mut self, v: ::std::string::String) {
        self.retryable = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retryable(&mut self) -> &mut ::std::string::String {
        if self.retryable.is_none() {
            self.retryable.set_default();
        };
        self.retryable.as_mut().unwrap()
    }

    // Take field
    pub fn take_retryable(&mut self) -> ::std::string::String {
        self.retryable.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_retryable(&self) -> &str {
        match self.retryable.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_retryable_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.retryable
    }

    fn mut_retryable_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.retryable
    }

    // optional string abort = 3;

    pub fn clear_abort(&mut self) {
        self.abort.clear();
    }

    pub fn has_abort(&self) -> bool {
        self.abort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_abort(&mut self, v: ::std::string::String) {
        self.abort = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_abort(&mut self) -> &mut ::std::string::String {
        if self.abort.is_none() {
            self.abort.set_default();
        };
        self.abort.as_mut().unwrap()
    }

    // Take field
    pub fn take_abort(&mut self) -> ::std::string::String {
        self.abort.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_abort(&self) -> &str {
        match self.abort.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_abort_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.abort
    }

    fn mut_abort_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.abort
    }
}

impl ::protobuf::Message for KeyError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locked)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.retryable)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.abort)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.locked.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.retryable.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.abort.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.locked.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.retryable.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.abort.as_ref() {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyError {
    fn new() -> KeyError {
        KeyError::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LockInfo>>(
                    "locked",
                    KeyError::get_locked_for_reflect,
                    KeyError::mut_locked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "retryable",
                    KeyError::get_retryable_for_reflect,
                    KeyError::mut_retryable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "abort",
                    KeyError::get_abort_for_reflect,
                    KeyError::mut_abort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyError>(
                    "KeyError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyError {
    fn clear(&mut self) {
        self.clear_locked();
        self.clear_retryable();
        self.clear_abort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Context {
    // message fields
    region_id: ::std::option::Option<u64>,
    region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    read_quorum: ::std::option::Option<bool>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Context {}

impl Context {
    pub fn new() -> Context {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Context {
        static mut instance: ::protobuf::lazy::Lazy<Context> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Context,
        };
        unsafe {
            instance.get(Context::new)
        }
    }

    // optional uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = ::std::option::Option::None;
    }

    pub fn has_region_id(&self) -> bool {
        self.region_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = ::std::option::Option::Some(v);
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id.unwrap_or(0)
    }

    fn get_region_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.region_id
    }

    // optional .metapb.RegionEpoch region_epoch = 2;

    pub fn clear_region_epoch(&mut self) {
        self.region_epoch.clear();
    }

    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch.set_default();
        };
        self.region_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch.take().unwrap_or_else(|| super::metapb::RegionEpoch::new())
    }

    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| super::metapb::RegionEpoch::default_instance())
    }

    fn get_region_epoch_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &self.region_epoch
    }

    fn mut_region_epoch_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &mut self.region_epoch
    }

    // optional .metapb.Peer peer = 3;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        };
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_peer(&self) -> &super::metapb::Peer {
        self.peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.peer
    }

    // optional bool read_quorum = 4;

    pub fn clear_read_quorum(&mut self) {
        self.read_quorum = ::std::option::Option::None;
    }

    pub fn has_read_quorum(&self) -> bool {
        self.read_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_quorum(&mut self, v: bool) {
        self.read_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_read_quorum(&self) -> bool {
        self.read_quorum.unwrap_or(false)
    }

    fn get_read_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.read_quorum
    }

    fn mut_read_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.read_quorum
    }

    // optional uint64 term = 5;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    fn get_term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.term
    }
}

impl ::protobuf::Message for Context {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.region_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.read_quorum = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.term = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.region_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.read_quorum {
            my_size += 2;
        };
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.region_epoch.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.peer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.read_quorum {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.term {
            os.write_uint64(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Context {
    fn new() -> Context {
        Context::new()
    }

    fn descriptor_static(_: ::std::option::Option<Context>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    Context::get_region_id_for_reflect,
                    Context::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    Context::get_region_epoch_for_reflect,
                    Context::mut_region_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "peer",
                    Context::get_peer_for_reflect,
                    Context::mut_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read_quorum",
                    Context::get_read_quorum_for_reflect,
                    Context::mut_read_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    Context::get_term_for_reflect,
                    Context::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Context>(
                    "Context",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_region_epoch();
        self.clear_peer();
        self.clear_read_quorum();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Context {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdGetRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetRequest {}

impl CmdGetRequest {
    pub fn new() -> CmdGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetRequest,
        };
        unsafe {
            instance.get(CmdGetRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for CmdGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGetRequest {
    fn new() -> CmdGetRequest {
        CmdGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdGetRequest::get_key_for_reflect,
                    CmdGetRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    CmdGetRequest::get_version_for_reflect,
                    CmdGetRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetRequest>(
                    "CmdGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdGetResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetResponse {}

impl CmdGetResponse {
    pub fn new() -> CmdGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetResponse,
        };
        unsafe {
            instance.get(CmdGetResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CmdGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGetResponse {
    fn new() -> CmdGetResponse {
        CmdGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdGetResponse::get_error_for_reflect,
                    CmdGetResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CmdGetResponse::get_value_for_reflect,
                    CmdGetResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetResponse>(
                    "CmdGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdScanRequest {
    // message fields
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    limit: ::std::option::Option<u32>,
    version: ::std::option::Option<u64>,
    key_only: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanRequest {}

impl CmdScanRequest {
    pub fn new() -> CmdScanRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanRequest,
        };
        unsafe {
            instance.get(CmdScanRequest::new)
        }
    }

    // optional bytes start_key = 1;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_start_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.start_key
    }

    // optional uint32 limit = 2;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(0)
    }

    fn get_limit_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.limit
    }

    // optional uint64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional bool key_only = 4;

    pub fn clear_key_only(&mut self) {
        self.key_only = ::std::option::Option::None;
    }

    pub fn has_key_only(&self) -> bool {
        self.key_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = ::std::option::Option::Some(v);
    }

    pub fn get_key_only(&self) -> bool {
        self.key_only.unwrap_or(false)
    }

    fn get_key_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.key_only
    }

    fn mut_key_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.key_only
    }
}

impl ::protobuf::Message for CmdScanRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.limit = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.key_only = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.limit {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.key_only {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.limit {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.version {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.key_only {
            os.write_bool(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanRequest {
    fn new() -> CmdScanRequest {
        CmdScanRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "start_key",
                    CmdScanRequest::get_start_key_for_reflect,
                    CmdScanRequest::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit",
                    CmdScanRequest::get_limit_for_reflect,
                    CmdScanRequest::mut_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    CmdScanRequest::get_version_for_reflect,
                    CmdScanRequest::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "key_only",
                    CmdScanRequest::get_key_only_for_reflect,
                    CmdScanRequest::mut_key_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanRequest>(
                    "CmdScanRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanRequest {
    fn clear(&mut self) {
        self.clear_start_key();
        self.clear_limit();
        self.clear_version();
        self.clear_key_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdScanRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KvPair {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KvPair {}

impl KvPair {
    pub fn new() -> KvPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KvPair {
        static mut instance: ::protobuf::lazy::Lazy<KvPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KvPair,
        };
        unsafe {
            instance.get(KvPair::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for KvPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KvPair {
    fn new() -> KvPair {
        KvPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KvPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    KvPair::get_error_for_reflect,
                    KvPair::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KvPair::get_key_for_reflect,
                    KvPair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KvPair::get_value_for_reflect,
                    KvPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KvPair>(
                    "KvPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KvPair {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KvPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KvPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdScanResponse {
    // message fields
    pairs: ::protobuf::RepeatedField<KvPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanResponse {}

impl CmdScanResponse {
    pub fn new() -> CmdScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanResponse,
        };
        unsafe {
            instance.get(CmdScanResponse::new)
        }
    }

    // repeated .kvrpcpb.KvPair pairs = 1;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs(&self) -> &[KvPair] {
        &self.pairs
    }

    fn get_pairs_for_reflect(&self) -> &::protobuf::RepeatedField<KvPair> {
        &self.pairs
    }

    fn mut_pairs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }
}

impl ::protobuf::Message for CmdScanResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.pairs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.pairs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanResponse {
    fn new() -> CmdScanResponse {
        CmdScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KvPair>>(
                    "pairs",
                    CmdScanResponse::get_pairs_for_reflect,
                    CmdScanResponse::mut_pairs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanResponse>(
                    "CmdScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanResponse {
    fn clear(&mut self) {
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdScanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mutation {
    // message fields
    op: ::std::option::Option<Op>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mutation {}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(Mutation::new)
        }
    }

    // optional .kvrpcpb.Op op = 1;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Op) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> Op {
        self.op.unwrap_or(Op::Put)
    }

    fn get_op_for_reflect(&self) -> &::std::option::Option<Op> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::std::option::Option<Op> {
        &mut self.op
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for Mutation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.op = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.op {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Op>>(
                    "op",
                    Mutation::get_op_for_reflect,
                    Mutation::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Mutation::get_key_for_reflect,
                    Mutation::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Mutation::get_value_for_reflect,
                    Mutation::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdPrewriteRequest {
    // message fields
    mutations: ::protobuf::RepeatedField<Mutation>,
    primary_lock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_version: ::std::option::Option<u64>,
    lock_ttl: ::std::option::Option<u64>,
    skip_constraint_check: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPrewriteRequest {}

impl CmdPrewriteRequest {
    pub fn new() -> CmdPrewriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPrewriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdPrewriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPrewriteRequest,
        };
        unsafe {
            instance.get(CmdPrewriteRequest::new)
        }
    }

    // repeated .kvrpcpb.Mutation mutations = 1;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn get_mutations_for_reflect(&self) -> &::protobuf::RepeatedField<Mutation> {
        &self.mutations
    }

    fn mut_mutations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // optional bytes primary_lock = 2;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    pub fn has_primary_lock(&self) -> bool {
        self.primary_lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary_lock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.primary_lock.is_none() {
            self.primary_lock.set_default();
        };
        self.primary_lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::std::vec::Vec<u8> {
        self.primary_lock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary_lock(&self) -> &[u8] {
        match self.primary_lock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_primary_lock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.primary_lock
    }

    fn mut_primary_lock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.primary_lock
    }

    // optional uint64 start_version = 3;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    fn get_start_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_version
    }

    // optional uint64 lock_ttl = 4;

    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = ::std::option::Option::None;
    }

    pub fn has_lock_ttl(&self) -> bool {
        self.lock_ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = ::std::option::Option::Some(v);
    }

    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl.unwrap_or(0)
    }

    fn get_lock_ttl_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lock_ttl
    }

    fn mut_lock_ttl_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lock_ttl
    }

    // optional bool skip_constraint_check = 5;

    pub fn clear_skip_constraint_check(&mut self) {
        self.skip_constraint_check = ::std::option::Option::None;
    }

    pub fn has_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skip_constraint_check(&mut self, v: bool) {
        self.skip_constraint_check = ::std::option::Option::Some(v);
    }

    pub fn get_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check.unwrap_or(false)
    }

    fn get_skip_constraint_check_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.skip_constraint_check
    }

    fn mut_skip_constraint_check_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.skip_constraint_check
    }
}

impl ::protobuf::Message for CmdPrewriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary_lock)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.lock_ttl = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.skip_constraint_check = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.mutations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.primary_lock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.start_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lock_ttl {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.skip_constraint_check {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.mutations {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.primary_lock.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.start_version {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.lock_ttl {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.skip_constraint_check {
            os.write_bool(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdPrewriteRequest {
    fn new() -> CmdPrewriteRequest {
        CmdPrewriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPrewriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mutation>>(
                    "mutations",
                    CmdPrewriteRequest::get_mutations_for_reflect,
                    CmdPrewriteRequest::mut_mutations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "primary_lock",
                    CmdPrewriteRequest::get_primary_lock_for_reflect,
                    CmdPrewriteRequest::mut_primary_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CmdPrewriteRequest::get_start_version_for_reflect,
                    CmdPrewriteRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_ttl",
                    CmdPrewriteRequest::get_lock_ttl_for_reflect,
                    CmdPrewriteRequest::mut_lock_ttl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "skip_constraint_check",
                    CmdPrewriteRequest::get_skip_constraint_check_for_reflect,
                    CmdPrewriteRequest::mut_skip_constraint_check_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPrewriteRequest>(
                    "CmdPrewriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPrewriteRequest {
    fn clear(&mut self) {
        self.clear_mutations();
        self.clear_primary_lock();
        self.clear_start_version();
        self.clear_lock_ttl();
        self.clear_skip_constraint_check();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdPrewriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdPrewriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdPrewriteResponse {
    // message fields
    errors: ::protobuf::RepeatedField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPrewriteResponse {}

impl CmdPrewriteResponse {
    pub fn new() -> CmdPrewriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPrewriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdPrewriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPrewriteResponse,
        };
        unsafe {
            instance.get(CmdPrewriteResponse::new)
        }
    }

    // repeated .kvrpcpb.KeyError errors = 1;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<KeyError>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<KeyError> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[KeyError] {
        &self.errors
    }

    fn get_errors_for_reflect(&self) -> &::protobuf::RepeatedField<KeyError> {
        &self.errors
    }

    fn mut_errors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }
}

impl ::protobuf::Message for CmdPrewriteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.errors {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdPrewriteResponse {
    fn new() -> CmdPrewriteResponse {
        CmdPrewriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPrewriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "errors",
                    CmdPrewriteResponse::get_errors_for_reflect,
                    CmdPrewriteResponse::mut_errors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPrewriteResponse>(
                    "CmdPrewriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPrewriteResponse {
    fn clear(&mut self) {
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdPrewriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdPrewriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdCommitRequest {
    // message fields
    start_version: ::std::option::Option<u64>,
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    commit_version: ::std::option::Option<u64>,
    binlog: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitRequest {}

impl CmdCommitRequest {
    pub fn new() -> CmdCommitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitRequest,
        };
        unsafe {
            instance.get(CmdCommitRequest::new)
        }
    }

    // optional uint64 start_version = 1;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    fn get_start_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_version
    }

    // repeated bytes keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // optional uint64 commit_version = 3;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version.unwrap_or(0)
    }

    fn get_commit_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit_version
    }

    // optional bytes binlog = 4;

    pub fn clear_binlog(&mut self) {
        self.binlog.clear();
    }

    pub fn has_binlog(&self) -> bool {
        self.binlog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_binlog(&mut self, v: ::std::vec::Vec<u8>) {
        self.binlog = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_binlog(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.binlog.is_none() {
            self.binlog.set_default();
        };
        self.binlog.as_mut().unwrap()
    }

    // Take field
    pub fn take_binlog(&mut self) -> ::std::vec::Vec<u8> {
        self.binlog.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_binlog(&self) -> &[u8] {
        match self.binlog.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_binlog_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.binlog
    }

    fn mut_binlog_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.binlog
    }
}

impl ::protobuf::Message for CmdCommitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.commit_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.binlog)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if let Some(v) = self.commit_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.binlog.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_version {
            os.write_uint64(1, v)?;
        };
        for v in &self.keys {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.commit_version {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.binlog.as_ref() {
            os.write_bytes(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitRequest {
    fn new() -> CmdCommitRequest {
        CmdCommitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CmdCommitRequest::get_start_version_for_reflect,
                    CmdCommitRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    CmdCommitRequest::get_keys_for_reflect,
                    CmdCommitRequest::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    CmdCommitRequest::get_commit_version_for_reflect,
                    CmdCommitRequest::mut_commit_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "binlog",
                    CmdCommitRequest::get_binlog_for_reflect,
                    CmdCommitRequest::mut_binlog_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitRequest>(
                    "CmdCommitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitRequest {
    fn clear(&mut self) {
        self.clear_start_version();
        self.clear_keys();
        self.clear_commit_version();
        self.clear_binlog();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdCommitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdCommitRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdCommitResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitResponse {}

impl CmdCommitResponse {
    pub fn new() -> CmdCommitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitResponse,
        };
        unsafe {
            instance.get(CmdCommitResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdCommitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitResponse {
    fn new() -> CmdCommitResponse {
        CmdCommitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdCommitResponse::get_error_for_reflect,
                    CmdCommitResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitResponse>(
                    "CmdCommitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdCommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdCommitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdBatchRollbackRequest {
    // message fields
    start_version: ::std::option::Option<u64>,
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchRollbackRequest {}

impl CmdBatchRollbackRequest {
    pub fn new() -> CmdBatchRollbackRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchRollbackRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchRollbackRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchRollbackRequest,
        };
        unsafe {
            instance.get(CmdBatchRollbackRequest::new)
        }
    }

    // optional uint64 start_version = 1;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    fn get_start_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_version
    }

    // repeated bytes keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CmdBatchRollbackRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_version {
            os.write_uint64(1, v)?;
        };
        for v in &self.keys {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchRollbackRequest {
    fn new() -> CmdBatchRollbackRequest {
        CmdBatchRollbackRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchRollbackRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CmdBatchRollbackRequest::get_start_version_for_reflect,
                    CmdBatchRollbackRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    CmdBatchRollbackRequest::get_keys_for_reflect,
                    CmdBatchRollbackRequest::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchRollbackRequest>(
                    "CmdBatchRollbackRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchRollbackRequest {
    fn clear(&mut self) {
        self.clear_start_version();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdBatchRollbackRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdBatchRollbackRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdBatchRollbackResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchRollbackResponse {}

impl CmdBatchRollbackResponse {
    pub fn new() -> CmdBatchRollbackResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchRollbackResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchRollbackResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchRollbackResponse,
        };
        unsafe {
            instance.get(CmdBatchRollbackResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdBatchRollbackResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchRollbackResponse {
    fn new() -> CmdBatchRollbackResponse {
        CmdBatchRollbackResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchRollbackResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdBatchRollbackResponse::get_error_for_reflect,
                    CmdBatchRollbackResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchRollbackResponse>(
                    "CmdBatchRollbackResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchRollbackResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdBatchRollbackResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdBatchRollbackResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdCleanupRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCleanupRequest {}

impl CmdCleanupRequest {
    pub fn new() -> CmdCleanupRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCleanupRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdCleanupRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCleanupRequest,
        };
        unsafe {
            instance.get(CmdCleanupRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional uint64 start_version = 2;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    fn get_start_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_version
    }
}

impl ::protobuf::Message for CmdCleanupRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.start_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.start_version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCleanupRequest {
    fn new() -> CmdCleanupRequest {
        CmdCleanupRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCleanupRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdCleanupRequest::get_key_for_reflect,
                    CmdCleanupRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CmdCleanupRequest::get_start_version_for_reflect,
                    CmdCleanupRequest::mut_start_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCleanupRequest>(
                    "CmdCleanupRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCleanupRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_start_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdCleanupRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdCleanupRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdCleanupResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    commit_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCleanupResponse {}

impl CmdCleanupResponse {
    pub fn new() -> CmdCleanupResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCleanupResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdCleanupResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCleanupResponse,
        };
        unsafe {
            instance.get(CmdCleanupResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // optional uint64 commit_version = 2;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version.unwrap_or(0)
    }

    fn get_commit_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit_version
    }
}

impl ::protobuf::Message for CmdCleanupResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.commit_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.commit_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.commit_version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCleanupResponse {
    fn new() -> CmdCleanupResponse {
        CmdCleanupResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCleanupResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdCleanupResponse::get_error_for_reflect,
                    CmdCleanupResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    CmdCleanupResponse::get_commit_version_for_reflect,
                    CmdCleanupResponse::mut_commit_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCleanupResponse>(
                    "CmdCleanupResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCleanupResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdCleanupResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdCleanupResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdBatchGetRequest {
    // message fields
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchGetRequest {}

impl CmdBatchGetRequest {
    pub fn new() -> CmdBatchGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchGetRequest,
        };
        unsafe {
            instance.get(CmdBatchGetRequest::new)
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for CmdBatchGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchGetRequest {
    fn new() -> CmdBatchGetRequest {
        CmdBatchGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keys",
                    CmdBatchGetRequest::get_keys_for_reflect,
                    CmdBatchGetRequest::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    CmdBatchGetRequest::get_version_for_reflect,
                    CmdBatchGetRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchGetRequest>(
                    "CmdBatchGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchGetRequest {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdBatchGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdBatchGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdBatchGetResponse {
    // message fields
    pairs: ::protobuf::RepeatedField<KvPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchGetResponse {}

impl CmdBatchGetResponse {
    pub fn new() -> CmdBatchGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchGetResponse,
        };
        unsafe {
            instance.get(CmdBatchGetResponse::new)
        }
    }

    // repeated .kvrpcpb.KvPair pairs = 1;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs(&self) -> &[KvPair] {
        &self.pairs
    }

    fn get_pairs_for_reflect(&self) -> &::protobuf::RepeatedField<KvPair> {
        &self.pairs
    }

    fn mut_pairs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }
}

impl ::protobuf::Message for CmdBatchGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.pairs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.pairs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchGetResponse {
    fn new() -> CmdBatchGetResponse {
        CmdBatchGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KvPair>>(
                    "pairs",
                    CmdBatchGetResponse::get_pairs_for_reflect,
                    CmdBatchGetResponse::mut_pairs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchGetResponse>(
                    "CmdBatchGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchGetResponse {
    fn clear(&mut self) {
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdBatchGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdBatchGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdScanLockRequest {
    // message fields
    max_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanLockRequest {}

impl CmdScanLockRequest {
    pub fn new() -> CmdScanLockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanLockRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanLockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanLockRequest,
        };
        unsafe {
            instance.get(CmdScanLockRequest::new)
        }
    }

    // optional uint64 max_version = 1;

    pub fn clear_max_version(&mut self) {
        self.max_version = ::std::option::Option::None;
    }

    pub fn has_max_version(&self) -> bool {
        self.max_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_version(&mut self, v: u64) {
        self.max_version = ::std::option::Option::Some(v);
    }

    pub fn get_max_version(&self) -> u64 {
        self.max_version.unwrap_or(0)
    }

    fn get_max_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.max_version
    }

    fn mut_max_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.max_version
    }
}

impl ::protobuf::Message for CmdScanLockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.max_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.max_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_version {
            os.write_uint64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanLockRequest {
    fn new() -> CmdScanLockRequest {
        CmdScanLockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanLockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "max_version",
                    CmdScanLockRequest::get_max_version_for_reflect,
                    CmdScanLockRequest::mut_max_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanLockRequest>(
                    "CmdScanLockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanLockRequest {
    fn clear(&mut self) {
        self.clear_max_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdScanLockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdScanLockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdScanLockResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    locks: ::protobuf::RepeatedField<LockInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanLockResponse {}

impl CmdScanLockResponse {
    pub fn new() -> CmdScanLockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanLockResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanLockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanLockResponse,
        };
        unsafe {
            instance.get(CmdScanLockResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // repeated .kvrpcpb.LockInfo locks = 2;

    pub fn clear_locks(&mut self) {
        self.locks.clear();
    }

    // Param is passed by value, moved
    pub fn set_locks(&mut self, v: ::protobuf::RepeatedField<LockInfo>) {
        self.locks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locks(&mut self) -> &mut ::protobuf::RepeatedField<LockInfo> {
        &mut self.locks
    }

    // Take field
    pub fn take_locks(&mut self) -> ::protobuf::RepeatedField<LockInfo> {
        ::std::mem::replace(&mut self.locks, ::protobuf::RepeatedField::new())
    }

    pub fn get_locks(&self) -> &[LockInfo] {
        &self.locks
    }

    fn get_locks_for_reflect(&self) -> &::protobuf::RepeatedField<LockInfo> {
        &self.locks
    }

    fn mut_locks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LockInfo> {
        &mut self.locks
    }
}

impl ::protobuf::Message for CmdScanLockResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locks)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.locks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.locks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanLockResponse {
    fn new() -> CmdScanLockResponse {
        CmdScanLockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanLockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdScanLockResponse::get_error_for_reflect,
                    CmdScanLockResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LockInfo>>(
                    "locks",
                    CmdScanLockResponse::get_locks_for_reflect,
                    CmdScanLockResponse::mut_locks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanLockResponse>(
                    "CmdScanLockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanLockResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_locks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdScanLockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdScanLockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdResolveLockRequest {
    // message fields
    start_version: ::std::option::Option<u64>,
    commit_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdResolveLockRequest {}

impl CmdResolveLockRequest {
    pub fn new() -> CmdResolveLockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdResolveLockRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdResolveLockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdResolveLockRequest,
        };
        unsafe {
            instance.get(CmdResolveLockRequest::new)
        }
    }

    // optional uint64 start_version = 1;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    fn get_start_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_version
    }

    // optional uint64 commit_version = 2;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version.unwrap_or(0)
    }

    fn get_commit_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit_version
    }
}

impl ::protobuf::Message for CmdResolveLockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.commit_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.commit_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_version {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.commit_version {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdResolveLockRequest {
    fn new() -> CmdResolveLockRequest {
        CmdResolveLockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdResolveLockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CmdResolveLockRequest::get_start_version_for_reflect,
                    CmdResolveLockRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    CmdResolveLockRequest::get_commit_version_for_reflect,
                    CmdResolveLockRequest::mut_commit_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdResolveLockRequest>(
                    "CmdResolveLockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdResolveLockRequest {
    fn clear(&mut self) {
        self.clear_start_version();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdResolveLockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdResolveLockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdResolveLockResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdResolveLockResponse {}

impl CmdResolveLockResponse {
    pub fn new() -> CmdResolveLockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdResolveLockResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdResolveLockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdResolveLockResponse,
        };
        unsafe {
            instance.get(CmdResolveLockResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdResolveLockResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdResolveLockResponse {
    fn new() -> CmdResolveLockResponse {
        CmdResolveLockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdResolveLockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdResolveLockResponse::get_error_for_reflect,
                    CmdResolveLockResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdResolveLockResponse>(
                    "CmdResolveLockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdResolveLockResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdResolveLockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdResolveLockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdGCRequest {
    // message fields
    safe_point: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGCRequest {}

impl CmdGCRequest {
    pub fn new() -> CmdGCRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGCRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdGCRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGCRequest,
        };
        unsafe {
            instance.get(CmdGCRequest::new)
        }
    }

    // optional uint64 safe_point = 1;

    pub fn clear_safe_point(&mut self) {
        self.safe_point = ::std::option::Option::None;
    }

    pub fn has_safe_point(&self) -> bool {
        self.safe_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = ::std::option::Option::Some(v);
    }

    pub fn get_safe_point(&self) -> u64 {
        self.safe_point.unwrap_or(0)
    }

    fn get_safe_point_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.safe_point
    }

    fn mut_safe_point_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.safe_point
    }
}

impl ::protobuf::Message for CmdGCRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.safe_point = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.safe_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.safe_point {
            os.write_uint64(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGCRequest {
    fn new() -> CmdGCRequest {
        CmdGCRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGCRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "safe_point",
                    CmdGCRequest::get_safe_point_for_reflect,
                    CmdGCRequest::mut_safe_point_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGCRequest>(
                    "CmdGCRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGCRequest {
    fn clear(&mut self) {
        self.clear_safe_point();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGCRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGCRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdGCResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGCResponse {}

impl CmdGCResponse {
    pub fn new() -> CmdGCResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGCResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdGCResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGCResponse,
        };
        unsafe {
            instance.get(CmdGCResponse::new)
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdGCResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGCResponse {
    fn new() -> CmdGCResponse {
        CmdGCResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGCResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CmdGCResponse::get_error_for_reflect,
                    CmdGCResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGCResponse>(
                    "CmdGCResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGCResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdGCResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdGCResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawGetRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawGetRequest {}

impl CmdRawGetRequest {
    pub fn new() -> CmdRawGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawGetRequest,
        };
        unsafe {
            instance.get(CmdRawGetRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }
}

impl ::protobuf::Message for CmdRawGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawGetRequest {
    fn new() -> CmdRawGetRequest {
        CmdRawGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdRawGetRequest::get_key_for_reflect,
                    CmdRawGetRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawGetRequest>(
                    "CmdRawGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawGetRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawGetResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawGetResponse {}

impl CmdRawGetResponse {
    pub fn new() -> CmdRawGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawGetResponse,
        };
        unsafe {
            instance.get(CmdRawGetResponse::new)
        }
    }

    // optional string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CmdRawGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawGetResponse {
    fn new() -> CmdRawGetResponse {
        CmdRawGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CmdRawGetResponse::get_error_for_reflect,
                    CmdRawGetResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CmdRawGetResponse::get_value_for_reflect,
                    CmdRawGetResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawGetResponse>(
                    "CmdRawGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawGetResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawPutRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawPutRequest {}

impl CmdRawPutRequest {
    pub fn new() -> CmdRawPutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawPutRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawPutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawPutRequest,
        };
        unsafe {
            instance.get(CmdRawPutRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CmdRawPutRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawPutRequest {
    fn new() -> CmdRawPutRequest {
        CmdRawPutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawPutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdRawPutRequest::get_key_for_reflect,
                    CmdRawPutRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CmdRawPutRequest::get_value_for_reflect,
                    CmdRawPutRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawPutRequest>(
                    "CmdRawPutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawPutRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawPutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawPutRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawPutResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawPutResponse {}

impl CmdRawPutResponse {
    pub fn new() -> CmdRawPutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawPutResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawPutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawPutResponse,
        };
        unsafe {
            instance.get(CmdRawPutResponse::new)
        }
    }

    // optional string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdRawPutResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawPutResponse {
    fn new() -> CmdRawPutResponse {
        CmdRawPutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawPutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CmdRawPutResponse::get_error_for_reflect,
                    CmdRawPutResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawPutResponse>(
                    "CmdRawPutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawPutResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawPutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawPutResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawDeleteRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawDeleteRequest {}

impl CmdRawDeleteRequest {
    pub fn new() -> CmdRawDeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawDeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawDeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawDeleteRequest,
        };
        unsafe {
            instance.get(CmdRawDeleteRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }
}

impl ::protobuf::Message for CmdRawDeleteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawDeleteRequest {
    fn new() -> CmdRawDeleteRequest {
        CmdRawDeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawDeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CmdRawDeleteRequest::get_key_for_reflect,
                    CmdRawDeleteRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawDeleteRequest>(
                    "CmdRawDeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawDeleteRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawDeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawDeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CmdRawDeleteResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRawDeleteResponse {}

impl CmdRawDeleteResponse {
    pub fn new() -> CmdRawDeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRawDeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdRawDeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRawDeleteResponse,
        };
        unsafe {
            instance.get(CmdRawDeleteResponse::new)
        }
    }

    // optional string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }
}

impl ::protobuf::Message for CmdRawDeleteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRawDeleteResponse {
    fn new() -> CmdRawDeleteResponse {
        CmdRawDeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRawDeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CmdRawDeleteResponse::get_error_for_reflect,
                    CmdRawDeleteResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRawDeleteResponse>(
                    "CmdRawDeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRawDeleteResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CmdRawDeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CmdRawDeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    field_type: ::std::option::Option<MessageType>,
    context: ::protobuf::SingularPtrField<Context>,
    cmd_get_req: ::protobuf::SingularPtrField<CmdGetRequest>,
    cmd_scan_req: ::protobuf::SingularPtrField<CmdScanRequest>,
    cmd_prewrite_req: ::protobuf::SingularPtrField<CmdPrewriteRequest>,
    cmd_commit_req: ::protobuf::SingularPtrField<CmdCommitRequest>,
    cmd_cleanup_req: ::protobuf::SingularPtrField<CmdCleanupRequest>,
    cmd_batch_get_req: ::protobuf::SingularPtrField<CmdBatchGetRequest>,
    cmd_batch_rollback_req: ::protobuf::SingularPtrField<CmdBatchRollbackRequest>,
    cmd_scan_lock_req: ::protobuf::SingularPtrField<CmdScanLockRequest>,
    cmd_resolve_lock_req: ::protobuf::SingularPtrField<CmdResolveLockRequest>,
    cmd_gc_req: ::protobuf::SingularPtrField<CmdGCRequest>,
    cmd_raw_get_req: ::protobuf::SingularPtrField<CmdRawGetRequest>,
    cmd_raw_put_req: ::protobuf::SingularPtrField<CmdRawPutRequest>,
    cmd_raw_delete_req: ::protobuf::SingularPtrField<CmdRawDeleteRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // optional .kvrpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type.unwrap_or(MessageType::CmdGet)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MessageType> {
        &mut self.field_type
    }

    // optional .kvrpcpb.Context context = 2;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // optional .kvrpcpb.CmdGetRequest cmd_get_req = 3;

    pub fn clear_cmd_get_req(&mut self) {
        self.cmd_get_req.clear();
    }

    pub fn has_cmd_get_req(&self) -> bool {
        self.cmd_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_req(&mut self, v: CmdGetRequest) {
        self.cmd_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_get_req(&mut self) -> &mut CmdGetRequest {
        if self.cmd_get_req.is_none() {
            self.cmd_get_req.set_default();
        };
        self.cmd_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_req(&mut self) -> CmdGetRequest {
        self.cmd_get_req.take().unwrap_or_else(|| CmdGetRequest::new())
    }

    pub fn get_cmd_get_req(&self) -> &CmdGetRequest {
        self.cmd_get_req.as_ref().unwrap_or_else(|| CmdGetRequest::default_instance())
    }

    fn get_cmd_get_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdGetRequest> {
        &self.cmd_get_req
    }

    fn mut_cmd_get_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdGetRequest> {
        &mut self.cmd_get_req
    }

    // optional .kvrpcpb.CmdScanRequest cmd_scan_req = 4;

    pub fn clear_cmd_scan_req(&mut self) {
        self.cmd_scan_req.clear();
    }

    pub fn has_cmd_scan_req(&self) -> bool {
        self.cmd_scan_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_req(&mut self, v: CmdScanRequest) {
        self.cmd_scan_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_req(&mut self) -> &mut CmdScanRequest {
        if self.cmd_scan_req.is_none() {
            self.cmd_scan_req.set_default();
        };
        self.cmd_scan_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_req(&mut self) -> CmdScanRequest {
        self.cmd_scan_req.take().unwrap_or_else(|| CmdScanRequest::new())
    }

    pub fn get_cmd_scan_req(&self) -> &CmdScanRequest {
        self.cmd_scan_req.as_ref().unwrap_or_else(|| CmdScanRequest::default_instance())
    }

    fn get_cmd_scan_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdScanRequest> {
        &self.cmd_scan_req
    }

    fn mut_cmd_scan_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdScanRequest> {
        &mut self.cmd_scan_req
    }

    // optional .kvrpcpb.CmdPrewriteRequest cmd_prewrite_req = 5;

    pub fn clear_cmd_prewrite_req(&mut self) {
        self.cmd_prewrite_req.clear();
    }

    pub fn has_cmd_prewrite_req(&self) -> bool {
        self.cmd_prewrite_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_prewrite_req(&mut self, v: CmdPrewriteRequest) {
        self.cmd_prewrite_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_prewrite_req(&mut self) -> &mut CmdPrewriteRequest {
        if self.cmd_prewrite_req.is_none() {
            self.cmd_prewrite_req.set_default();
        };
        self.cmd_prewrite_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_req(&mut self) -> CmdPrewriteRequest {
        self.cmd_prewrite_req.take().unwrap_or_else(|| CmdPrewriteRequest::new())
    }

    pub fn get_cmd_prewrite_req(&self) -> &CmdPrewriteRequest {
        self.cmd_prewrite_req.as_ref().unwrap_or_else(|| CmdPrewriteRequest::default_instance())
    }

    fn get_cmd_prewrite_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdPrewriteRequest> {
        &self.cmd_prewrite_req
    }

    fn mut_cmd_prewrite_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdPrewriteRequest> {
        &mut self.cmd_prewrite_req
    }

    // optional .kvrpcpb.CmdCommitRequest cmd_commit_req = 6;

    pub fn clear_cmd_commit_req(&mut self) {
        self.cmd_commit_req.clear();
    }

    pub fn has_cmd_commit_req(&self) -> bool {
        self.cmd_commit_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_req(&mut self, v: CmdCommitRequest) {
        self.cmd_commit_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_req(&mut self) -> &mut CmdCommitRequest {
        if self.cmd_commit_req.is_none() {
            self.cmd_commit_req.set_default();
        };
        self.cmd_commit_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_req(&mut self) -> CmdCommitRequest {
        self.cmd_commit_req.take().unwrap_or_else(|| CmdCommitRequest::new())
    }

    pub fn get_cmd_commit_req(&self) -> &CmdCommitRequest {
        self.cmd_commit_req.as_ref().unwrap_or_else(|| CmdCommitRequest::default_instance())
    }

    fn get_cmd_commit_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdCommitRequest> {
        &self.cmd_commit_req
    }

    fn mut_cmd_commit_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdCommitRequest> {
        &mut self.cmd_commit_req
    }

    // optional .kvrpcpb.CmdCleanupRequest cmd_cleanup_req = 7;

    pub fn clear_cmd_cleanup_req(&mut self) {
        self.cmd_cleanup_req.clear();
    }

    pub fn has_cmd_cleanup_req(&self) -> bool {
        self.cmd_cleanup_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_cleanup_req(&mut self, v: CmdCleanupRequest) {
        self.cmd_cleanup_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_cleanup_req(&mut self) -> &mut CmdCleanupRequest {
        if self.cmd_cleanup_req.is_none() {
            self.cmd_cleanup_req.set_default();
        };
        self.cmd_cleanup_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_req(&mut self) -> CmdCleanupRequest {
        self.cmd_cleanup_req.take().unwrap_or_else(|| CmdCleanupRequest::new())
    }

    pub fn get_cmd_cleanup_req(&self) -> &CmdCleanupRequest {
        self.cmd_cleanup_req.as_ref().unwrap_or_else(|| CmdCleanupRequest::default_instance())
    }

    fn get_cmd_cleanup_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdCleanupRequest> {
        &self.cmd_cleanup_req
    }

    fn mut_cmd_cleanup_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdCleanupRequest> {
        &mut self.cmd_cleanup_req
    }

    // optional .kvrpcpb.CmdBatchGetRequest cmd_batch_get_req = 10;

    pub fn clear_cmd_batch_get_req(&mut self) {
        self.cmd_batch_get_req.clear();
    }

    pub fn has_cmd_batch_get_req(&self) -> bool {
        self.cmd_batch_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_get_req(&mut self, v: CmdBatchGetRequest) {
        self.cmd_batch_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_get_req(&mut self) -> &mut CmdBatchGetRequest {
        if self.cmd_batch_get_req.is_none() {
            self.cmd_batch_get_req.set_default();
        };
        self.cmd_batch_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_req(&mut self) -> CmdBatchGetRequest {
        self.cmd_batch_get_req.take().unwrap_or_else(|| CmdBatchGetRequest::new())
    }

    pub fn get_cmd_batch_get_req(&self) -> &CmdBatchGetRequest {
        self.cmd_batch_get_req.as_ref().unwrap_or_else(|| CmdBatchGetRequest::default_instance())
    }

    fn get_cmd_batch_get_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdBatchGetRequest> {
        &self.cmd_batch_get_req
    }

    fn mut_cmd_batch_get_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdBatchGetRequest> {
        &mut self.cmd_batch_get_req
    }

    // optional .kvrpcpb.CmdBatchRollbackRequest cmd_batch_rollback_req = 11;

    pub fn clear_cmd_batch_rollback_req(&mut self) {
        self.cmd_batch_rollback_req.clear();
    }

    pub fn has_cmd_batch_rollback_req(&self) -> bool {
        self.cmd_batch_rollback_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_rollback_req(&mut self, v: CmdBatchRollbackRequest) {
        self.cmd_batch_rollback_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_rollback_req(&mut self) -> &mut CmdBatchRollbackRequest {
        if self.cmd_batch_rollback_req.is_none() {
            self.cmd_batch_rollback_req.set_default();
        };
        self.cmd_batch_rollback_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_rollback_req(&mut self) -> CmdBatchRollbackRequest {
        self.cmd_batch_rollback_req.take().unwrap_or_else(|| CmdBatchRollbackRequest::new())
    }

    pub fn get_cmd_batch_rollback_req(&self) -> &CmdBatchRollbackRequest {
        self.cmd_batch_rollback_req.as_ref().unwrap_or_else(|| CmdBatchRollbackRequest::default_instance())
    }

    fn get_cmd_batch_rollback_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdBatchRollbackRequest> {
        &self.cmd_batch_rollback_req
    }

    fn mut_cmd_batch_rollback_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdBatchRollbackRequest> {
        &mut self.cmd_batch_rollback_req
    }

    // optional .kvrpcpb.CmdScanLockRequest cmd_scan_lock_req = 12;

    pub fn clear_cmd_scan_lock_req(&mut self) {
        self.cmd_scan_lock_req.clear();
    }

    pub fn has_cmd_scan_lock_req(&self) -> bool {
        self.cmd_scan_lock_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_lock_req(&mut self, v: CmdScanLockRequest) {
        self.cmd_scan_lock_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_lock_req(&mut self) -> &mut CmdScanLockRequest {
        if self.cmd_scan_lock_req.is_none() {
            self.cmd_scan_lock_req.set_default();
        };
        self.cmd_scan_lock_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_lock_req(&mut self) -> CmdScanLockRequest {
        self.cmd_scan_lock_req.take().unwrap_or_else(|| CmdScanLockRequest::new())
    }

    pub fn get_cmd_scan_lock_req(&self) -> &CmdScanLockRequest {
        self.cmd_scan_lock_req.as_ref().unwrap_or_else(|| CmdScanLockRequest::default_instance())
    }

    fn get_cmd_scan_lock_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdScanLockRequest> {
        &self.cmd_scan_lock_req
    }

    fn mut_cmd_scan_lock_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdScanLockRequest> {
        &mut self.cmd_scan_lock_req
    }

    // optional .kvrpcpb.CmdResolveLockRequest cmd_resolve_lock_req = 13;

    pub fn clear_cmd_resolve_lock_req(&mut self) {
        self.cmd_resolve_lock_req.clear();
    }

    pub fn has_cmd_resolve_lock_req(&self) -> bool {
        self.cmd_resolve_lock_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_resolve_lock_req(&mut self, v: CmdResolveLockRequest) {
        self.cmd_resolve_lock_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_resolve_lock_req(&mut self) -> &mut CmdResolveLockRequest {
        if self.cmd_resolve_lock_req.is_none() {
            self.cmd_resolve_lock_req.set_default();
        };
        self.cmd_resolve_lock_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_resolve_lock_req(&mut self) -> CmdResolveLockRequest {
        self.cmd_resolve_lock_req.take().unwrap_or_else(|| CmdResolveLockRequest::new())
    }

    pub fn get_cmd_resolve_lock_req(&self) -> &CmdResolveLockRequest {
        self.cmd_resolve_lock_req.as_ref().unwrap_or_else(|| CmdResolveLockRequest::default_instance())
    }

    fn get_cmd_resolve_lock_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdResolveLockRequest> {
        &self.cmd_resolve_lock_req
    }

    fn mut_cmd_resolve_lock_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdResolveLockRequest> {
        &mut self.cmd_resolve_lock_req
    }

    // optional .kvrpcpb.CmdGCRequest cmd_gc_req = 14;

    pub fn clear_cmd_gc_req(&mut self) {
        self.cmd_gc_req.clear();
    }

    pub fn has_cmd_gc_req(&self) -> bool {
        self.cmd_gc_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_gc_req(&mut self, v: CmdGCRequest) {
        self.cmd_gc_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_gc_req(&mut self) -> &mut CmdGCRequest {
        if self.cmd_gc_req.is_none() {
            self.cmd_gc_req.set_default();
        };
        self.cmd_gc_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_gc_req(&mut self) -> CmdGCRequest {
        self.cmd_gc_req.take().unwrap_or_else(|| CmdGCRequest::new())
    }

    pub fn get_cmd_gc_req(&self) -> &CmdGCRequest {
        self.cmd_gc_req.as_ref().unwrap_or_else(|| CmdGCRequest::default_instance())
    }

    fn get_cmd_gc_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdGCRequest> {
        &self.cmd_gc_req
    }

    fn mut_cmd_gc_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdGCRequest> {
        &mut self.cmd_gc_req
    }

    // optional .kvrpcpb.CmdRawGetRequest cmd_raw_get_req = 256;

    pub fn clear_cmd_raw_get_req(&mut self) {
        self.cmd_raw_get_req.clear();
    }

    pub fn has_cmd_raw_get_req(&self) -> bool {
        self.cmd_raw_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_get_req(&mut self, v: CmdRawGetRequest) {
        self.cmd_raw_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_get_req(&mut self) -> &mut CmdRawGetRequest {
        if self.cmd_raw_get_req.is_none() {
            self.cmd_raw_get_req.set_default();
        };
        self.cmd_raw_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_get_req(&mut self) -> CmdRawGetRequest {
        self.cmd_raw_get_req.take().unwrap_or_else(|| CmdRawGetRequest::new())
    }

    pub fn get_cmd_raw_get_req(&self) -> &CmdRawGetRequest {
        self.cmd_raw_get_req.as_ref().unwrap_or_else(|| CmdRawGetRequest::default_instance())
    }

    fn get_cmd_raw_get_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawGetRequest> {
        &self.cmd_raw_get_req
    }

    fn mut_cmd_raw_get_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawGetRequest> {
        &mut self.cmd_raw_get_req
    }

    // optional .kvrpcpb.CmdRawPutRequest cmd_raw_put_req = 257;

    pub fn clear_cmd_raw_put_req(&mut self) {
        self.cmd_raw_put_req.clear();
    }

    pub fn has_cmd_raw_put_req(&self) -> bool {
        self.cmd_raw_put_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_put_req(&mut self, v: CmdRawPutRequest) {
        self.cmd_raw_put_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_put_req(&mut self) -> &mut CmdRawPutRequest {
        if self.cmd_raw_put_req.is_none() {
            self.cmd_raw_put_req.set_default();
        };
        self.cmd_raw_put_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_put_req(&mut self) -> CmdRawPutRequest {
        self.cmd_raw_put_req.take().unwrap_or_else(|| CmdRawPutRequest::new())
    }

    pub fn get_cmd_raw_put_req(&self) -> &CmdRawPutRequest {
        self.cmd_raw_put_req.as_ref().unwrap_or_else(|| CmdRawPutRequest::default_instance())
    }

    fn get_cmd_raw_put_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawPutRequest> {
        &self.cmd_raw_put_req
    }

    fn mut_cmd_raw_put_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawPutRequest> {
        &mut self.cmd_raw_put_req
    }

    // optional .kvrpcpb.CmdRawDeleteRequest cmd_raw_delete_req = 258;

    pub fn clear_cmd_raw_delete_req(&mut self) {
        self.cmd_raw_delete_req.clear();
    }

    pub fn has_cmd_raw_delete_req(&self) -> bool {
        self.cmd_raw_delete_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_delete_req(&mut self, v: CmdRawDeleteRequest) {
        self.cmd_raw_delete_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_delete_req(&mut self) -> &mut CmdRawDeleteRequest {
        if self.cmd_raw_delete_req.is_none() {
            self.cmd_raw_delete_req.set_default();
        };
        self.cmd_raw_delete_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_delete_req(&mut self) -> CmdRawDeleteRequest {
        self.cmd_raw_delete_req.take().unwrap_or_else(|| CmdRawDeleteRequest::new())
    }

    pub fn get_cmd_raw_delete_req(&self) -> &CmdRawDeleteRequest {
        self.cmd_raw_delete_req.as_ref().unwrap_or_else(|| CmdRawDeleteRequest::default_instance())
    }

    fn get_cmd_raw_delete_req_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawDeleteRequest> {
        &self.cmd_raw_delete_req
    }

    fn mut_cmd_raw_delete_req_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawDeleteRequest> {
        &mut self.cmd_raw_delete_req
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_get_req)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_req)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_prewrite_req)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_req)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_cleanup_req)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_get_req)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_rollback_req)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_lock_req)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_resolve_lock_req)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_gc_req)?;
                },
                256 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_get_req)?;
                },
                257 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_put_req)?;
                },
                258 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_delete_req)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_get_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_scan_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_prewrite_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_commit_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_cleanup_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_batch_get_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_batch_rollback_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_scan_lock_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_resolve_lock_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_gc_req.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_get_req.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_put_req.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_delete_req.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.context.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_get_req.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_scan_req.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_prewrite_req.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_commit_req.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_cleanup_req.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_batch_get_req.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_batch_rollback_req.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_scan_lock_req.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_resolve_lock_req.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_gc_req.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_get_req.as_ref() {
            os.write_tag(256, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_put_req.as_ref() {
            os.write_tag(257, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_delete_req.as_ref() {
            os.write_tag(258, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Request::get_field_type_for_reflect,
                    Request::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    Request::get_context_for_reflect,
                    Request::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdGetRequest>>(
                    "cmd_get_req",
                    Request::get_cmd_get_req_for_reflect,
                    Request::mut_cmd_get_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdScanRequest>>(
                    "cmd_scan_req",
                    Request::get_cmd_scan_req_for_reflect,
                    Request::mut_cmd_scan_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdPrewriteRequest>>(
                    "cmd_prewrite_req",
                    Request::get_cmd_prewrite_req_for_reflect,
                    Request::mut_cmd_prewrite_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdCommitRequest>>(
                    "cmd_commit_req",
                    Request::get_cmd_commit_req_for_reflect,
                    Request::mut_cmd_commit_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdCleanupRequest>>(
                    "cmd_cleanup_req",
                    Request::get_cmd_cleanup_req_for_reflect,
                    Request::mut_cmd_cleanup_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdBatchGetRequest>>(
                    "cmd_batch_get_req",
                    Request::get_cmd_batch_get_req_for_reflect,
                    Request::mut_cmd_batch_get_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdBatchRollbackRequest>>(
                    "cmd_batch_rollback_req",
                    Request::get_cmd_batch_rollback_req_for_reflect,
                    Request::mut_cmd_batch_rollback_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdScanLockRequest>>(
                    "cmd_scan_lock_req",
                    Request::get_cmd_scan_lock_req_for_reflect,
                    Request::mut_cmd_scan_lock_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdResolveLockRequest>>(
                    "cmd_resolve_lock_req",
                    Request::get_cmd_resolve_lock_req_for_reflect,
                    Request::mut_cmd_resolve_lock_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdGCRequest>>(
                    "cmd_gc_req",
                    Request::get_cmd_gc_req_for_reflect,
                    Request::mut_cmd_gc_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawGetRequest>>(
                    "cmd_raw_get_req",
                    Request::get_cmd_raw_get_req_for_reflect,
                    Request::mut_cmd_raw_get_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawPutRequest>>(
                    "cmd_raw_put_req",
                    Request::get_cmd_raw_put_req_for_reflect,
                    Request::mut_cmd_raw_put_req_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawDeleteRequest>>(
                    "cmd_raw_delete_req",
                    Request::get_cmd_raw_delete_req_for_reflect,
                    Request::mut_cmd_raw_delete_req_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_context();
        self.clear_cmd_get_req();
        self.clear_cmd_scan_req();
        self.clear_cmd_prewrite_req();
        self.clear_cmd_commit_req();
        self.clear_cmd_cleanup_req();
        self.clear_cmd_batch_get_req();
        self.clear_cmd_batch_rollback_req();
        self.clear_cmd_scan_lock_req();
        self.clear_cmd_resolve_lock_req();
        self.clear_cmd_gc_req();
        self.clear_cmd_raw_get_req();
        self.clear_cmd_raw_put_req();
        self.clear_cmd_raw_delete_req();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    field_type: ::std::option::Option<MessageType>,
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    cmd_get_resp: ::protobuf::SingularPtrField<CmdGetResponse>,
    cmd_scan_resp: ::protobuf::SingularPtrField<CmdScanResponse>,
    cmd_prewrite_resp: ::protobuf::SingularPtrField<CmdPrewriteResponse>,
    cmd_commit_resp: ::protobuf::SingularPtrField<CmdCommitResponse>,
    cmd_cleanup_resp: ::protobuf::SingularPtrField<CmdCleanupResponse>,
    cmd_batch_get_resp: ::protobuf::SingularPtrField<CmdBatchGetResponse>,
    cmd_batch_rollback_resp: ::protobuf::SingularPtrField<CmdBatchRollbackResponse>,
    cmd_scan_lock_resp: ::protobuf::SingularPtrField<CmdScanLockResponse>,
    cmd_resolve_lock_resp: ::protobuf::SingularPtrField<CmdResolveLockResponse>,
    cmd_gc_resp: ::protobuf::SingularPtrField<CmdGCResponse>,
    cmd_raw_get_resp: ::protobuf::SingularPtrField<CmdRawGetResponse>,
    cmd_raw_put_resp: ::protobuf::SingularPtrField<CmdRawPutResponse>,
    cmd_raw_delete_resp: ::protobuf::SingularPtrField<CmdRawDeleteResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // optional .kvrpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type.unwrap_or(MessageType::CmdGet)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MessageType> {
        &mut self.field_type
    }

    // optional .errorpb.Error region_error = 2;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        };
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // optional .kvrpcpb.CmdGetResponse cmd_get_resp = 3;

    pub fn clear_cmd_get_resp(&mut self) {
        self.cmd_get_resp.clear();
    }

    pub fn has_cmd_get_resp(&self) -> bool {
        self.cmd_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_resp(&mut self, v: CmdGetResponse) {
        self.cmd_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_get_resp(&mut self) -> &mut CmdGetResponse {
        if self.cmd_get_resp.is_none() {
            self.cmd_get_resp.set_default();
        };
        self.cmd_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_resp(&mut self) -> CmdGetResponse {
        self.cmd_get_resp.take().unwrap_or_else(|| CmdGetResponse::new())
    }

    pub fn get_cmd_get_resp(&self) -> &CmdGetResponse {
        self.cmd_get_resp.as_ref().unwrap_or_else(|| CmdGetResponse::default_instance())
    }

    fn get_cmd_get_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdGetResponse> {
        &self.cmd_get_resp
    }

    fn mut_cmd_get_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdGetResponse> {
        &mut self.cmd_get_resp
    }

    // optional .kvrpcpb.CmdScanResponse cmd_scan_resp = 4;

    pub fn clear_cmd_scan_resp(&mut self) {
        self.cmd_scan_resp.clear();
    }

    pub fn has_cmd_scan_resp(&self) -> bool {
        self.cmd_scan_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_resp(&mut self, v: CmdScanResponse) {
        self.cmd_scan_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_resp(&mut self) -> &mut CmdScanResponse {
        if self.cmd_scan_resp.is_none() {
            self.cmd_scan_resp.set_default();
        };
        self.cmd_scan_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_resp(&mut self) -> CmdScanResponse {
        self.cmd_scan_resp.take().unwrap_or_else(|| CmdScanResponse::new())
    }

    pub fn get_cmd_scan_resp(&self) -> &CmdScanResponse {
        self.cmd_scan_resp.as_ref().unwrap_or_else(|| CmdScanResponse::default_instance())
    }

    fn get_cmd_scan_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdScanResponse> {
        &self.cmd_scan_resp
    }

    fn mut_cmd_scan_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdScanResponse> {
        &mut self.cmd_scan_resp
    }

    // optional .kvrpcpb.CmdPrewriteResponse cmd_prewrite_resp = 5;

    pub fn clear_cmd_prewrite_resp(&mut self) {
        self.cmd_prewrite_resp.clear();
    }

    pub fn has_cmd_prewrite_resp(&self) -> bool {
        self.cmd_prewrite_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_prewrite_resp(&mut self, v: CmdPrewriteResponse) {
        self.cmd_prewrite_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_prewrite_resp(&mut self) -> &mut CmdPrewriteResponse {
        if self.cmd_prewrite_resp.is_none() {
            self.cmd_prewrite_resp.set_default();
        };
        self.cmd_prewrite_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_resp(&mut self) -> CmdPrewriteResponse {
        self.cmd_prewrite_resp.take().unwrap_or_else(|| CmdPrewriteResponse::new())
    }

    pub fn get_cmd_prewrite_resp(&self) -> &CmdPrewriteResponse {
        self.cmd_prewrite_resp.as_ref().unwrap_or_else(|| CmdPrewriteResponse::default_instance())
    }

    fn get_cmd_prewrite_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdPrewriteResponse> {
        &self.cmd_prewrite_resp
    }

    fn mut_cmd_prewrite_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdPrewriteResponse> {
        &mut self.cmd_prewrite_resp
    }

    // optional .kvrpcpb.CmdCommitResponse cmd_commit_resp = 6;

    pub fn clear_cmd_commit_resp(&mut self) {
        self.cmd_commit_resp.clear();
    }

    pub fn has_cmd_commit_resp(&self) -> bool {
        self.cmd_commit_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_resp(&mut self, v: CmdCommitResponse) {
        self.cmd_commit_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_resp(&mut self) -> &mut CmdCommitResponse {
        if self.cmd_commit_resp.is_none() {
            self.cmd_commit_resp.set_default();
        };
        self.cmd_commit_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_resp(&mut self) -> CmdCommitResponse {
        self.cmd_commit_resp.take().unwrap_or_else(|| CmdCommitResponse::new())
    }

    pub fn get_cmd_commit_resp(&self) -> &CmdCommitResponse {
        self.cmd_commit_resp.as_ref().unwrap_or_else(|| CmdCommitResponse::default_instance())
    }

    fn get_cmd_commit_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdCommitResponse> {
        &self.cmd_commit_resp
    }

    fn mut_cmd_commit_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdCommitResponse> {
        &mut self.cmd_commit_resp
    }

    // optional .kvrpcpb.CmdCleanupResponse cmd_cleanup_resp = 7;

    pub fn clear_cmd_cleanup_resp(&mut self) {
        self.cmd_cleanup_resp.clear();
    }

    pub fn has_cmd_cleanup_resp(&self) -> bool {
        self.cmd_cleanup_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_cleanup_resp(&mut self, v: CmdCleanupResponse) {
        self.cmd_cleanup_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_cleanup_resp(&mut self) -> &mut CmdCleanupResponse {
        if self.cmd_cleanup_resp.is_none() {
            self.cmd_cleanup_resp.set_default();
        };
        self.cmd_cleanup_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_resp(&mut self) -> CmdCleanupResponse {
        self.cmd_cleanup_resp.take().unwrap_or_else(|| CmdCleanupResponse::new())
    }

    pub fn get_cmd_cleanup_resp(&self) -> &CmdCleanupResponse {
        self.cmd_cleanup_resp.as_ref().unwrap_or_else(|| CmdCleanupResponse::default_instance())
    }

    fn get_cmd_cleanup_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdCleanupResponse> {
        &self.cmd_cleanup_resp
    }

    fn mut_cmd_cleanup_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdCleanupResponse> {
        &mut self.cmd_cleanup_resp
    }

    // optional .kvrpcpb.CmdBatchGetResponse cmd_batch_get_resp = 10;

    pub fn clear_cmd_batch_get_resp(&mut self) {
        self.cmd_batch_get_resp.clear();
    }

    pub fn has_cmd_batch_get_resp(&self) -> bool {
        self.cmd_batch_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_get_resp(&mut self, v: CmdBatchGetResponse) {
        self.cmd_batch_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_get_resp(&mut self) -> &mut CmdBatchGetResponse {
        if self.cmd_batch_get_resp.is_none() {
            self.cmd_batch_get_resp.set_default();
        };
        self.cmd_batch_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_resp(&mut self) -> CmdBatchGetResponse {
        self.cmd_batch_get_resp.take().unwrap_or_else(|| CmdBatchGetResponse::new())
    }

    pub fn get_cmd_batch_get_resp(&self) -> &CmdBatchGetResponse {
        self.cmd_batch_get_resp.as_ref().unwrap_or_else(|| CmdBatchGetResponse::default_instance())
    }

    fn get_cmd_batch_get_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdBatchGetResponse> {
        &self.cmd_batch_get_resp
    }

    fn mut_cmd_batch_get_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdBatchGetResponse> {
        &mut self.cmd_batch_get_resp
    }

    // optional .kvrpcpb.CmdBatchRollbackResponse cmd_batch_rollback_resp = 11;

    pub fn clear_cmd_batch_rollback_resp(&mut self) {
        self.cmd_batch_rollback_resp.clear();
    }

    pub fn has_cmd_batch_rollback_resp(&self) -> bool {
        self.cmd_batch_rollback_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_rollback_resp(&mut self, v: CmdBatchRollbackResponse) {
        self.cmd_batch_rollback_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_rollback_resp(&mut self) -> &mut CmdBatchRollbackResponse {
        if self.cmd_batch_rollback_resp.is_none() {
            self.cmd_batch_rollback_resp.set_default();
        };
        self.cmd_batch_rollback_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_rollback_resp(&mut self) -> CmdBatchRollbackResponse {
        self.cmd_batch_rollback_resp.take().unwrap_or_else(|| CmdBatchRollbackResponse::new())
    }

    pub fn get_cmd_batch_rollback_resp(&self) -> &CmdBatchRollbackResponse {
        self.cmd_batch_rollback_resp.as_ref().unwrap_or_else(|| CmdBatchRollbackResponse::default_instance())
    }

    fn get_cmd_batch_rollback_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdBatchRollbackResponse> {
        &self.cmd_batch_rollback_resp
    }

    fn mut_cmd_batch_rollback_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdBatchRollbackResponse> {
        &mut self.cmd_batch_rollback_resp
    }

    // optional .kvrpcpb.CmdScanLockResponse cmd_scan_lock_resp = 12;

    pub fn clear_cmd_scan_lock_resp(&mut self) {
        self.cmd_scan_lock_resp.clear();
    }

    pub fn has_cmd_scan_lock_resp(&self) -> bool {
        self.cmd_scan_lock_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_lock_resp(&mut self, v: CmdScanLockResponse) {
        self.cmd_scan_lock_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_lock_resp(&mut self) -> &mut CmdScanLockResponse {
        if self.cmd_scan_lock_resp.is_none() {
            self.cmd_scan_lock_resp.set_default();
        };
        self.cmd_scan_lock_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_lock_resp(&mut self) -> CmdScanLockResponse {
        self.cmd_scan_lock_resp.take().unwrap_or_else(|| CmdScanLockResponse::new())
    }

    pub fn get_cmd_scan_lock_resp(&self) -> &CmdScanLockResponse {
        self.cmd_scan_lock_resp.as_ref().unwrap_or_else(|| CmdScanLockResponse::default_instance())
    }

    fn get_cmd_scan_lock_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdScanLockResponse> {
        &self.cmd_scan_lock_resp
    }

    fn mut_cmd_scan_lock_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdScanLockResponse> {
        &mut self.cmd_scan_lock_resp
    }

    // optional .kvrpcpb.CmdResolveLockResponse cmd_resolve_lock_resp = 13;

    pub fn clear_cmd_resolve_lock_resp(&mut self) {
        self.cmd_resolve_lock_resp.clear();
    }

    pub fn has_cmd_resolve_lock_resp(&self) -> bool {
        self.cmd_resolve_lock_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_resolve_lock_resp(&mut self, v: CmdResolveLockResponse) {
        self.cmd_resolve_lock_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_resolve_lock_resp(&mut self) -> &mut CmdResolveLockResponse {
        if self.cmd_resolve_lock_resp.is_none() {
            self.cmd_resolve_lock_resp.set_default();
        };
        self.cmd_resolve_lock_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_resolve_lock_resp(&mut self) -> CmdResolveLockResponse {
        self.cmd_resolve_lock_resp.take().unwrap_or_else(|| CmdResolveLockResponse::new())
    }

    pub fn get_cmd_resolve_lock_resp(&self) -> &CmdResolveLockResponse {
        self.cmd_resolve_lock_resp.as_ref().unwrap_or_else(|| CmdResolveLockResponse::default_instance())
    }

    fn get_cmd_resolve_lock_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdResolveLockResponse> {
        &self.cmd_resolve_lock_resp
    }

    fn mut_cmd_resolve_lock_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdResolveLockResponse> {
        &mut self.cmd_resolve_lock_resp
    }

    // optional .kvrpcpb.CmdGCResponse cmd_gc_resp = 14;

    pub fn clear_cmd_gc_resp(&mut self) {
        self.cmd_gc_resp.clear();
    }

    pub fn has_cmd_gc_resp(&self) -> bool {
        self.cmd_gc_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_gc_resp(&mut self, v: CmdGCResponse) {
        self.cmd_gc_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_gc_resp(&mut self) -> &mut CmdGCResponse {
        if self.cmd_gc_resp.is_none() {
            self.cmd_gc_resp.set_default();
        };
        self.cmd_gc_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_gc_resp(&mut self) -> CmdGCResponse {
        self.cmd_gc_resp.take().unwrap_or_else(|| CmdGCResponse::new())
    }

    pub fn get_cmd_gc_resp(&self) -> &CmdGCResponse {
        self.cmd_gc_resp.as_ref().unwrap_or_else(|| CmdGCResponse::default_instance())
    }

    fn get_cmd_gc_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdGCResponse> {
        &self.cmd_gc_resp
    }

    fn mut_cmd_gc_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdGCResponse> {
        &mut self.cmd_gc_resp
    }

    // optional .kvrpcpb.CmdRawGetResponse cmd_raw_get_resp = 256;

    pub fn clear_cmd_raw_get_resp(&mut self) {
        self.cmd_raw_get_resp.clear();
    }

    pub fn has_cmd_raw_get_resp(&self) -> bool {
        self.cmd_raw_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_get_resp(&mut self, v: CmdRawGetResponse) {
        self.cmd_raw_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_get_resp(&mut self) -> &mut CmdRawGetResponse {
        if self.cmd_raw_get_resp.is_none() {
            self.cmd_raw_get_resp.set_default();
        };
        self.cmd_raw_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_get_resp(&mut self) -> CmdRawGetResponse {
        self.cmd_raw_get_resp.take().unwrap_or_else(|| CmdRawGetResponse::new())
    }

    pub fn get_cmd_raw_get_resp(&self) -> &CmdRawGetResponse {
        self.cmd_raw_get_resp.as_ref().unwrap_or_else(|| CmdRawGetResponse::default_instance())
    }

    fn get_cmd_raw_get_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawGetResponse> {
        &self.cmd_raw_get_resp
    }

    fn mut_cmd_raw_get_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawGetResponse> {
        &mut self.cmd_raw_get_resp
    }

    // optional .kvrpcpb.CmdRawPutResponse cmd_raw_put_resp = 257;

    pub fn clear_cmd_raw_put_resp(&mut self) {
        self.cmd_raw_put_resp.clear();
    }

    pub fn has_cmd_raw_put_resp(&self) -> bool {
        self.cmd_raw_put_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_put_resp(&mut self, v: CmdRawPutResponse) {
        self.cmd_raw_put_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_put_resp(&mut self) -> &mut CmdRawPutResponse {
        if self.cmd_raw_put_resp.is_none() {
            self.cmd_raw_put_resp.set_default();
        };
        self.cmd_raw_put_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_put_resp(&mut self) -> CmdRawPutResponse {
        self.cmd_raw_put_resp.take().unwrap_or_else(|| CmdRawPutResponse::new())
    }

    pub fn get_cmd_raw_put_resp(&self) -> &CmdRawPutResponse {
        self.cmd_raw_put_resp.as_ref().unwrap_or_else(|| CmdRawPutResponse::default_instance())
    }

    fn get_cmd_raw_put_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawPutResponse> {
        &self.cmd_raw_put_resp
    }

    fn mut_cmd_raw_put_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawPutResponse> {
        &mut self.cmd_raw_put_resp
    }

    // optional .kvrpcpb.CmdRawDeleteResponse cmd_raw_delete_resp = 258;

    pub fn clear_cmd_raw_delete_resp(&mut self) {
        self.cmd_raw_delete_resp.clear();
    }

    pub fn has_cmd_raw_delete_resp(&self) -> bool {
        self.cmd_raw_delete_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_raw_delete_resp(&mut self, v: CmdRawDeleteResponse) {
        self.cmd_raw_delete_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_raw_delete_resp(&mut self) -> &mut CmdRawDeleteResponse {
        if self.cmd_raw_delete_resp.is_none() {
            self.cmd_raw_delete_resp.set_default();
        };
        self.cmd_raw_delete_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_raw_delete_resp(&mut self) -> CmdRawDeleteResponse {
        self.cmd_raw_delete_resp.take().unwrap_or_else(|| CmdRawDeleteResponse::new())
    }

    pub fn get_cmd_raw_delete_resp(&self) -> &CmdRawDeleteResponse {
        self.cmd_raw_delete_resp.as_ref().unwrap_or_else(|| CmdRawDeleteResponse::default_instance())
    }

    fn get_cmd_raw_delete_resp_for_reflect(&self) -> &::protobuf::SingularPtrField<CmdRawDeleteResponse> {
        &self.cmd_raw_delete_resp
    }

    fn mut_cmd_raw_delete_resp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CmdRawDeleteResponse> {
        &mut self.cmd_raw_delete_resp
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_get_resp)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_resp)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_prewrite_resp)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_resp)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_cleanup_resp)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_get_resp)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_rollback_resp)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_lock_resp)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_resolve_lock_resp)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_gc_resp)?;
                },
                256 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_get_resp)?;
                },
                257 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_put_resp)?;
                },
                258 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_raw_delete_resp)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_get_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_scan_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_prewrite_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_commit_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_cleanup_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_batch_get_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_batch_rollback_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_scan_lock_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_resolve_lock_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_gc_resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_get_resp.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_put_resp.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cmd_raw_delete_resp.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.region_error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_get_resp.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_scan_resp.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_prewrite_resp.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_commit_resp.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_cleanup_resp.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_batch_get_resp.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_batch_rollback_resp.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_scan_lock_resp.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_resolve_lock_resp.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_gc_resp.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_get_resp.as_ref() {
            os.write_tag(256, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_put_resp.as_ref() {
            os.write_tag(257, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cmd_raw_delete_resp.as_ref() {
            os.write_tag(258, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "type",
                    Response::get_field_type_for_reflect,
                    Response::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    Response::get_region_error_for_reflect,
                    Response::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdGetResponse>>(
                    "cmd_get_resp",
                    Response::get_cmd_get_resp_for_reflect,
                    Response::mut_cmd_get_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdScanResponse>>(
                    "cmd_scan_resp",
                    Response::get_cmd_scan_resp_for_reflect,
                    Response::mut_cmd_scan_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdPrewriteResponse>>(
                    "cmd_prewrite_resp",
                    Response::get_cmd_prewrite_resp_for_reflect,
                    Response::mut_cmd_prewrite_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdCommitResponse>>(
                    "cmd_commit_resp",
                    Response::get_cmd_commit_resp_for_reflect,
                    Response::mut_cmd_commit_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdCleanupResponse>>(
                    "cmd_cleanup_resp",
                    Response::get_cmd_cleanup_resp_for_reflect,
                    Response::mut_cmd_cleanup_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdBatchGetResponse>>(
                    "cmd_batch_get_resp",
                    Response::get_cmd_batch_get_resp_for_reflect,
                    Response::mut_cmd_batch_get_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdBatchRollbackResponse>>(
                    "cmd_batch_rollback_resp",
                    Response::get_cmd_batch_rollback_resp_for_reflect,
                    Response::mut_cmd_batch_rollback_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdScanLockResponse>>(
                    "cmd_scan_lock_resp",
                    Response::get_cmd_scan_lock_resp_for_reflect,
                    Response::mut_cmd_scan_lock_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdResolveLockResponse>>(
                    "cmd_resolve_lock_resp",
                    Response::get_cmd_resolve_lock_resp_for_reflect,
                    Response::mut_cmd_resolve_lock_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdGCResponse>>(
                    "cmd_gc_resp",
                    Response::get_cmd_gc_resp_for_reflect,
                    Response::mut_cmd_gc_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawGetResponse>>(
                    "cmd_raw_get_resp",
                    Response::get_cmd_raw_get_resp_for_reflect,
                    Response::mut_cmd_raw_get_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawPutResponse>>(
                    "cmd_raw_put_resp",
                    Response::get_cmd_raw_put_resp_for_reflect,
                    Response::mut_cmd_raw_put_resp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CmdRawDeleteResponse>>(
                    "cmd_raw_delete_resp",
                    Response::get_cmd_raw_delete_resp_for_reflect,
                    Response::mut_cmd_raw_delete_resp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_region_error();
        self.clear_cmd_get_resp();
        self.clear_cmd_scan_resp();
        self.clear_cmd_prewrite_resp();
        self.clear_cmd_commit_resp();
        self.clear_cmd_cleanup_resp();
        self.clear_cmd_batch_get_resp();
        self.clear_cmd_batch_rollback_resp();
        self.clear_cmd_scan_lock_resp();
        self.clear_cmd_resolve_lock_resp();
        self.clear_cmd_gc_resp();
        self.clear_cmd_raw_get_resp();
        self.clear_cmd_raw_put_resp();
        self.clear_cmd_raw_delete_resp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    CmdGet = 0,
    CmdScan = 1,
    CmdPrewrite = 2,
    CmdCommit = 3,
    CmdCleanup = 4,
    CmdBatchGet = 7,
    CmdBatchRollback = 8,
    CmdScanLock = 9,
    CmdResolveLock = 10,
    CmdGC = 11,
    CmdRawGet = 256,
    CmdRawPut = 257,
    CmdRawDelete = 258,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::CmdGet),
            1 => ::std::option::Option::Some(MessageType::CmdScan),
            2 => ::std::option::Option::Some(MessageType::CmdPrewrite),
            3 => ::std::option::Option::Some(MessageType::CmdCommit),
            4 => ::std::option::Option::Some(MessageType::CmdCleanup),
            7 => ::std::option::Option::Some(MessageType::CmdBatchGet),
            8 => ::std::option::Option::Some(MessageType::CmdBatchRollback),
            9 => ::std::option::Option::Some(MessageType::CmdScanLock),
            10 => ::std::option::Option::Some(MessageType::CmdResolveLock),
            11 => ::std::option::Option::Some(MessageType::CmdGC),
            256 => ::std::option::Option::Some(MessageType::CmdRawGet),
            257 => ::std::option::Option::Some(MessageType::CmdRawPut),
            258 => ::std::option::Option::Some(MessageType::CmdRawDelete),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::CmdGet,
            MessageType::CmdScan,
            MessageType::CmdPrewrite,
            MessageType::CmdCommit,
            MessageType::CmdCleanup,
            MessageType::CmdBatchGet,
            MessageType::CmdBatchRollback,
            MessageType::CmdScanLock,
            MessageType::CmdResolveLock,
            MessageType::CmdGC,
            MessageType::CmdRawGet,
            MessageType::CmdRawPut,
            MessageType::CmdRawDelete,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
}

impl ::protobuf::ProtobufEnum for Op {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Op> {
        match value {
            0 => ::std::option::Option::Some(Op::Put),
            1 => ::std::option::Option::Some(Op::Del),
            2 => ::std::option::Option::Some(Op::Lock),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Op] = &[
            Op::Put,
            Op::Del,
            Op::Lock,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Op>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Op", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Op {
}

impl ::protobuf::reflect::ProtobufValue for Op {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x07, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x1a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x89, 0x01, 0x0a, 0x08,
    0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x72, 0x69, 0x6d,
    0x61, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b,
    0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x4c, 0x6f, 0x63, 0x6b, 0x12, 0x27, 0x0a, 0x0c, 0x6c,
    0x6f, 0x63, 0x6b, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0b, 0x6c, 0x6f, 0x63, 0x6b, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04,
    0xc8, 0xde, 0x1f, 0x00, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x1f, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x74,
    0x74, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6c, 0x6f, 0x63, 0x6b, 0x54, 0x74,
    0x6c, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x75, 0x0a, 0x08, 0x4b, 0x65, 0x79, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4c, 0x6f,
    0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x22,
    0x0a, 0x09, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x12, 0x1a, 0x0a, 0x05, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x05, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0xc7,
    0x01, 0x0a, 0x07, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x21, 0x0a, 0x09, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x36, 0x0a,
    0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x45, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x20, 0x0a, 0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65,
    0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x0b, 0x72, 0x65, 0x61, 0x64, 0x5f,
    0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x72, 0x65,
    0x61, 0x64, 0x51, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x18,
    0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x74, 0x65,
    0x72, 0x6d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x41, 0x0a, 0x0d, 0x43, 0x6d, 0x64, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x1e, 0x0a, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x4f, 0x0a, 0x0e, 0x43,
    0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x8a, 0x01, 0x0a,
    0x0e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x1b, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x05,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x1e, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x1f, 0x0a, 0x08, 0x6b, 0x65, 0x79, 0x5f,
    0x6f, 0x6e, 0x6c, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x6b, 0x65, 0x79, 0x4f,
    0x6e, 0x6c, 0x79, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x59, 0x0a, 0x06, 0x4b, 0x76, 0x50,
    0x61, 0x69, 0x72, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x22, 0x38, 0x0a, 0x0f, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x05, 0x70, 0x61, 0x69, 0x72, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62,
    0x2e, 0x4b, 0x76, 0x50, 0x61, 0x69, 0x72, 0x52, 0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x22, 0x55,
    0x0a, 0x08, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x02, 0x6f, 0x70,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62,
    0x2e, 0x4f, 0x70, 0x52, 0x02, 0x6f, 0x70, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x10, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
    0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xee, 0x01, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2f, 0x0a, 0x09,
    0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x09, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x21, 0x0a,
    0x0c, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0b, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x4c, 0x6f, 0x63, 0x6b,
    0x12, 0x29, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x1f, 0x0a, 0x08, 0x6c,
    0x6f, 0x63, 0x6b, 0x5f, 0x74, 0x74, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6c,
    0x6f, 0x63, 0x6b, 0x54, 0x74, 0x6c, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x38, 0x0a, 0x15,
    0x73, 0x6b, 0x69, 0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x74, 0x5f,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x13, 0x73, 0x6b, 0x69,
    0x70, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x40, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a,
    0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x52, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x22, 0x96, 0x01, 0x0a, 0x10, 0x43, 0x6d, 0x64,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a,
    0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x12, 0x2b, 0x0a, 0x0e,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x69, 0x6e,
    0x6c, 0x6f, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x62, 0x69, 0x6e, 0x6c, 0x6f,
    0x67, 0x22, 0x3c, 0x0a, 0x11, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22,
    0x58, 0x0a, 0x17, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62,
    0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x0d, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42,
    0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0c, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x22, 0x43, 0x0a, 0x18, 0x43, 0x6d, 0x64,
    0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b,
    0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x50,
    0x0a, 0x11, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x29, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00,
    0x22, 0x6a, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12,
    0x2b, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x48, 0x0a, 0x12,
    0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c,
    0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x12, 0x1e, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x3c, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a,
    0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x76, 0x50, 0x61, 0x69, 0x72, 0x52, 0x05, 0x70,
    0x61, 0x69, 0x72, 0x73, 0x22, 0x3b, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c,
    0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x25, 0x0a, 0x0b, 0x6d, 0x61,
    0x78, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0a, 0x6d, 0x61, 0x78, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f,
    0x00, 0x22, 0x67, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f, 0x63, 0x6b,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x12, 0x27, 0x0a, 0x05, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4c, 0x6f, 0x63, 0x6b, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x05, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x22, 0x6f, 0x0a, 0x15, 0x43, 0x6d,
    0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x2b,
    0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x22, 0x41, 0x0a, 0x16, 0x43,
    0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b,
    0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x33,
    0x0a, 0x0c, 0x43, 0x6d, 0x64, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23,
    0x0a, 0x0a, 0x73, 0x61, 0x66, 0x65, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x09, 0x73, 0x61, 0x66, 0x65, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x42, 0x04, 0xc8,
    0xde, 0x1f, 0x00, 0x22, 0x38, 0x0a, 0x0d, 0x43, 0x6d, 0x64, 0x47, 0x43, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65,
    0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x24, 0x0a,
    0x10, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x22, 0x3f, 0x0a, 0x11, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x47, 0x65, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x14,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x22, 0x3a, 0x0a, 0x10, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x50, 0x75,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0x29, 0x0a, 0x11, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x27, 0x0a, 0x13, 0x43,
    0x6d, 0x64, 0x52, 0x61, 0x77, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x03, 0x6b, 0x65, 0x79, 0x22, 0x2c, 0x0a, 0x14, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x22, 0xe3, 0x07, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x2a,
    0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x36, 0x0a, 0x0b, 0x63, 0x6d,
    0x64, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x16, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x71, 0x12, 0x39, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f, 0x72,
    0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63,
    0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x52, 0x0a, 0x63, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x12, 0x45, 0x0a,
    0x10, 0x63, 0x6d, 0x64, 0x5f, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x72, 0x65,
    0x71, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x0e, 0x63, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x52, 0x65, 0x71, 0x12, 0x3f, 0x0a, 0x0e, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x63, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x52, 0x65, 0x71, 0x12, 0x42, 0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c, 0x65,
    0x61, 0x6e, 0x75, 0x70, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61,
    0x6e, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0d, 0x63, 0x6d, 0x64, 0x43,
    0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x71, 0x12, 0x46, 0x0a, 0x11, 0x63, 0x6d, 0x64,
    0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
    0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x52, 0x0e, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x71, 0x12, 0x55, 0x0a, 0x16, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x72,
    0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x42,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x13, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c,
    0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x12, 0x46, 0x0a, 0x11, 0x63, 0x6d, 0x64, 0x5f,
    0x73, 0x63, 0x61, 0x6e, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x52, 0x0e, 0x63, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71,
    0x12, 0x4f, 0x0a, 0x14, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x5f,
    0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x11,
    0x63, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65,
    0x71, 0x12, 0x33, 0x0a, 0x0a, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x63, 0x5f, 0x72, 0x65, 0x71, 0x18,
    0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x43, 0x6d, 0x64, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x63, 0x6d,
    0x64, 0x47, 0x63, 0x52, 0x65, 0x71, 0x12, 0x41, 0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x61,
    0x77, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x80, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x19, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x61,
    0x77, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x63, 0x6d, 0x64,
    0x52, 0x61, 0x77, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x41, 0x0a, 0x0f, 0x63, 0x6d, 0x64,
    0x5f, 0x72, 0x61, 0x77, 0x5f, 0x70, 0x75, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x81, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c,
    0x63, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x12, 0x4a, 0x0a, 0x12,
    0x63, 0x6d, 0x64, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72,
    0x65, 0x71, 0x18, 0x82, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70,
    0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0f, 0x63, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x44,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x22, 0x92, 0x08, 0x0a, 0x08, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2e, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x42,
    0x04, 0xc8, 0xde, 0x1f, 0x00, 0x12, 0x31, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x0b, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x39, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f,
    0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a, 0x63, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x12, 0x3c, 0x0a, 0x0d, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f,
    0x72, 0x65, 0x73, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0b, 0x63, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x48, 0x0a, 0x11, 0x63, 0x6d, 0x64, 0x5f, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0f, 0x63, 0x6d, 0x64, 0x50,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x42, 0x0a, 0x0f, 0x63,
    0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
    0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x0d, 0x63, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12,
    0x45, 0x0a, 0x10, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x5f, 0x72,
    0x65, 0x73, 0x70, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70,
    0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x63, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e,
    0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x12, 0x49, 0x0a, 0x12, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61,
    0x74, 0x63, 0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64,
    0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x0f, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x58, 0x0a, 0x17, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x72,
    0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64,
    0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x14, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52,
    0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x12, 0x49, 0x0a, 0x12, 0x63,
    0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0f, 0x63, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f,
    0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x12, 0x52, 0x0a, 0x15, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x12, 0x63, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x12, 0x36, 0x0a, 0x0b, 0x63, 0x6d,
    0x64, 0x5f, 0x67, 0x63, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x16, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x43, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x47, 0x63, 0x52, 0x65,
    0x73, 0x70, 0x12, 0x44, 0x0a, 0x10, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x67, 0x65,
    0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x80, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0d, 0x63, 0x6d, 0x64, 0x52, 0x61,
    0x77, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x44, 0x0a, 0x10, 0x63, 0x6d, 0x64, 0x5f,
    0x72, 0x61, 0x77, 0x5f, 0x70, 0x75, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x81, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x0d, 0x63, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x4d,
    0x0a, 0x13, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x82, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x44, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x63, 0x6d, 0x64,
    0x52, 0x61, 0x77, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x2a, 0xe0, 0x01,
    0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a,
    0x06, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6d, 0x64,
    0x53, 0x63, 0x61, 0x6e, 0x10, 0x01, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x43, 0x6d, 0x64, 0x43, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x10, 0x03, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65,
    0x61, 0x6e, 0x75, 0x70, 0x10, 0x04, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x47, 0x65, 0x74, 0x10, 0x07, 0x12, 0x14, 0x0a, 0x10, 0x43, 0x6d, 0x64, 0x42, 0x61,
    0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x10, 0x08, 0x12, 0x0f, 0x0a,
    0x0b, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x4c, 0x6f, 0x63, 0x6b, 0x10, 0x09, 0x12, 0x12,
    0x0a, 0x0e, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x4c, 0x6f, 0x63, 0x6b,
    0x10, 0x0a, 0x12, 0x09, 0x0a, 0x05, 0x43, 0x6d, 0x64, 0x47, 0x43, 0x10, 0x0b, 0x12, 0x0e, 0x0a,
    0x09, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x47, 0x65, 0x74, 0x10, 0x80, 0x02, 0x12, 0x0e, 0x0a,
    0x09, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x50, 0x75, 0x74, 0x10, 0x81, 0x02, 0x12, 0x11, 0x0a,
    0x0c, 0x43, 0x6d, 0x64, 0x52, 0x61, 0x77, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x10, 0x82, 0x02,
    0x2a, 0x20, 0x0a, 0x02, 0x4f, 0x70, 0x12, 0x07, 0x0a, 0x03, 0x50, 0x75, 0x74, 0x10, 0x00, 0x12,
    0x07, 0x0a, 0x03, 0x44, 0x65, 0x6c, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x4c, 0x6f, 0x63, 0x6b,
    0x10, 0x02, 0x42, 0x0c, 0xe0, 0xe2, 0x1e, 0x01, 0xc8, 0xe2, 0x1e, 0x01, 0xd0, 0xe2, 0x1e, 0x01,
    0x4a, 0xe5, 0x5d, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xe8, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0f,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x04, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x07,
    0x1d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x00, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x07, 0x07, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x07, 0x07, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x07, 0x23, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x08, 0x00, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x12, 0x03, 0x08, 0x07, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03,
    0x12, 0x03, 0x08, 0x1f, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x09, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x09, 0x07, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x09, 0x07, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x02, 0x03, 0x12, 0x03, 0x09, 0x25, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0b,
    0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x05, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0d, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0f, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x0f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x10, 0x04, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x11, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x11, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x12,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x12, 0x04, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a, 0x31,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x13, 0x04, 0x1c, 0x22, 0x24, 0x20, 0x53, 0x63,
    0x61, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x72, 0x70, 0x68, 0x61, 0x6e, 0x20, 0x6c, 0x6f,
    0x63, 0x6b, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x13, 0x04, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x40, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x14, 0x04, 0x1d, 0x22, 0x33, 0x20, 0x52, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x72, 0x70, 0x68, 0x61, 0x6e, 0x20,
    0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x14, 0x04, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x14, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x15, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x15, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12,
    0x03, 0x15, 0x1a, 0x1c, 0x0a, 0xa9, 0x01, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1a,
    0x04, 0x1e, 0x1a, 0x9b, 0x01, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x33, 0x20, 0x72, 0x61,
    0x77, 0x20, 0x41, 0x50, 0x49, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72,
    0x61, 0x77, 0x20, 0x6b, 0x76, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x64, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x0a, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x4d, 0x56, 0x43, 0x43, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x79, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20,
    0x6d, 0x69, 0x78, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x1a, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x1b, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02,
    0x12, 0x03, 0x1b, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x1c,
    0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x1c, 0x1a, 0x1d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x20, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x20, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x21, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21,
    0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x21, 0x25, 0x43, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x21, 0x26, 0x42, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x21, 0x26,
    0x3a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x21, 0x26, 0x3a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x27, 0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x21, 0x3d, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x22, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x22, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x14,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x22, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x23, 0x04, 0x44, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x23, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x23, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x23,
    0x25, 0x43, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x23, 0x26, 0x42, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x23, 0x26, 0x3a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x26, 0x3a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x27, 0x39, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x23, 0x3d, 0x42, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x26, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x27, 0x04, 0x24, 0x22, 0x37, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x20, 0x6f, 0x72, 0x20, 0x63,
    0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x20,
    0x74, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x27, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x27, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x27, 0x22, 0x23, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x28, 0x04,
    0x43, 0x22, 0x31, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x72,
    0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x2e, 0x20,
    0x65, 0x2e, 0x67, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69,
    0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x28,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x16, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x28, 0x24, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x28, 0x25, 0x41, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x28, 0x25, 0x39, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x28, 0x25, 0x39,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x26, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x28, 0x3c, 0x41, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x29, 0x04, 0x43, 0x22, 0x1e, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x29, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x16, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x29, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x08, 0x12, 0x03, 0x29, 0x24, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x29, 0x25, 0x41, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x29, 0x25, 0x39, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x25, 0x39, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x29, 0x26, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x29, 0x3c, 0x41, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2c, 0x00, 0x32,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x04, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x2d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2d, 0x20, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d,
    0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2d, 0x31, 0x4f,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2d, 0x32,
    0x4e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x2d, 0x32, 0x46, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x2d, 0x32, 0x46, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x33, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x49, 0x4e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x2e, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x2e, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x2f,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x04, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x2f, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x30, 0x04, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x20, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x08, 0x12, 0x03, 0x30, 0x31, 0x4f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x30, 0x32, 0x4e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x30, 0x32, 0x46, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x30, 0x32, 0x46, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x30, 0x33, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x30, 0x49, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x31,
    0x04, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x08, 0x12, 0x03, 0x31, 0x31, 0x4f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x31, 0x32, 0x4e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x31, 0x32, 0x46, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x02, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x32, 0x46, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x02, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31,
    0x33, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x31, 0x49, 0x4e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x37, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x35, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x35, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x35, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x36, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x36, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x36, 0x21, 0x3f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x36, 0x22, 0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x36, 0x22, 0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x36, 0x22, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x23, 0x35, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x36, 0x39, 0x3e,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x39, 0x00, 0x3c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x39, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x3a, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x3a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3a, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x16, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x3b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x3b, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3b,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3e, 0x00, 0x43, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x3f, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x3f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x15,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x40, 0x04, 0x42, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x40, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x40, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x40, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08, 0x12, 0x03, 0x40,
    0x23, 0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x40, 0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x40, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x40, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x40, 0x3b, 0x40, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x41, 0x04, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x41, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x41, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x41, 0x23,
    0x41, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x41,
    0x24, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x41, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x41, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x05, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x41, 0x3b, 0x40, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x42, 0x04, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x42, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x42, 0x15, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x42,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x08, 0x12, 0x03, 0x42, 0x23, 0x41,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x42, 0x24,
    0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x42, 0x24, 0x38, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x42, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x25, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05,
    0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x42, 0x3b, 0x40, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x45, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x46, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x01, 0x12, 0x03, 0x47, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x47,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x16, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x47, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x48, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x48, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x48, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x48, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4b, 0x00, 0x4d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x4c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c,
    0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x1c, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x4f, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x05, 0x07, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x50, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x50, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x50, 0x0b,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x51, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x51, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x52, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x52, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x52, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x55, 0x00, 0x59, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x56, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x56, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56,
    0x13, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x56, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x08, 0x12, 0x03, 0x56, 0x1d, 0x3b, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x56, 0x1e, 0x3a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x56, 0x1e,
    0x32, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x56, 0x1e, 0x32, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x08, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x1f, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x56, 0x35, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x03, 0x57, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x57, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x13,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x58, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x58, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x58, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x58, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x5b, 0x00, 0x62, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x5c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x5c, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x2e,
    0x2f, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x04, 0x30, 0x1a, 0x12,
    0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6b, 0x65,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5e, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5e, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x02, 0x12, 0x03, 0x5f, 0x04, 0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x5f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x16,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x2e, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x08, 0x12, 0x03, 0x5f, 0x30, 0x4e, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5f, 0x31, 0x4d, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5f, 0x31, 0x45,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x5f, 0x31, 0x45, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5f, 0x32, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x48, 0x4d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x03, 0x12, 0x03, 0x60, 0x04, 0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x60,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x60, 0x16, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x60, 0x2e, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x08, 0x12, 0x03, 0x60, 0x30, 0x4e, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x09, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x60, 0x31, 0x4d, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x09, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x60, 0x31, 0x45, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x09, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x60,
    0x31, 0x45, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x60, 0x32, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x03, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x60, 0x48, 0x4d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04,
    0x12, 0x03, 0x61, 0x04, 0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x61, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x03, 0x61, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x03, 0x61, 0x16, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x03, 0x61, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x04, 0x08, 0x12, 0x03, 0x61, 0x30, 0x4e, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x09, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x61, 0x31, 0x4d, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x09, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x61, 0x31, 0x45, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x09, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x61, 0x31,
    0x45, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x61, 0x32, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x61, 0x48, 0x4d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x64,
    0x00, 0x66, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x64, 0x08, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x65, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x65, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x65, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x65, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x68, 0x00, 0x6d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x68, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x69, 0x04, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x69, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x69, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x08, 0x12, 0x03, 0x69, 0x28, 0x46, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0b, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x69, 0x29, 0x45,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0b, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x69,
    0x29, 0x3d, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0b, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x69, 0x29, 0x3d, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0b, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x2a, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0b, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x69, 0x40, 0x45, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x6a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6a,
    0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6a, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x04, 0x47, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x6b, 0x15, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x6b, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x08, 0x12, 0x03,
    0x6b, 0x28, 0x46, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0b, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x6b, 0x29, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0b, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x6b, 0x29, 0x3d, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0b, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x29, 0x3d, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0b, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x2a, 0x3c, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0b, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x40, 0x45, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x6c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x6c, 0x15, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x6c, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x6f, 0x00, 0x71, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x70, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x70, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x1e,
    0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x73, 0x00, 0x76, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x73, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x00, 0x12, 0x03, 0x74, 0x04, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x74, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x74,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x74, 0x14, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x74, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x08, 0x12, 0x03, 0x74, 0x26, 0x44, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x74, 0x27, 0x43, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x74, 0x27, 0x3b, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x74,
    0x27, 0x3b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x74, 0x28, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x74, 0x3e, 0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01,
    0x12, 0x03, 0x75, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x75, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x75, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x75, 0x14, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x75, 0x24, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0e, 0x12, 0x04, 0x78, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01,
    0x12, 0x03, 0x78, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x79,
    0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x79, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f,
    0x12, 0x04, 0x7c, 0x00, 0x7f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x7c,
    0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x04, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x7d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12,
    0x03, 0x7e, 0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7e,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7e, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7e, 0x15, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7e, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x08, 0x12, 0x03, 0x7e, 0x27, 0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0f,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x7e, 0x28, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x7e, 0x28, 0x3c, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x28, 0x3c,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x7e, 0x29, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x7e, 0x3f, 0x44, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x81, 0x01,
    0x00, 0x84, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08,
    0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x04, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x27, 0x28, 0x0a, 0x38, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x04, 0x48, 0x22, 0x2a, 0x20, 0x73, 0x65, 0x74, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x69, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0x83,
    0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x01,
    0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x01, 0x27,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x08, 0x12, 0x04, 0x83, 0x01, 0x29, 0x47,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x10, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x83, 0x01,
    0x2a, 0x46, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x10, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x04, 0x83, 0x01, 0x2a, 0x3e, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x10, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x83, 0x01, 0x2a, 0x3e, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x10, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x01, 0x2b, 0x3d, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x10, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x83, 0x01, 0x41,
    0x46, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0x86, 0x01, 0x00, 0x89, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x87, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x87, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04,
    0x88, 0x01, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x01,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x14,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1e, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x08, 0x12, 0x04, 0x88, 0x01, 0x20, 0x3e, 0x0a,
    0x10, 0x0a, 0x08, 0x04, 0x11, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x88, 0x01, 0x21,
    0x3d, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x11, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04,
    0x88, 0x01, 0x21, 0x35, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x11, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x21, 0x35, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x11, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x01, 0x22, 0x34, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x11, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01, 0x38, 0x3d,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x8b, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8c, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8c, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x8f, 0x01, 0x00,
    0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x04, 0x43, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x08, 0x12, 0x04, 0x90, 0x01, 0x24, 0x42, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x13, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x90, 0x01, 0x25, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04,
    0x13, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x90, 0x01, 0x25, 0x39, 0x0a, 0x12,
    0x0a, 0x0a, 0x04, 0x13, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01,
    0x25, 0x39, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x13, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x90, 0x01, 0x26, 0x38, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x13, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x3c, 0x41, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14,
    0x12, 0x06, 0x93, 0x01, 0x00, 0x96, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12,
    0x04, 0x93, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x94,
    0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0x94, 0x01, 0x0d,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x16, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1e, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x06, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12,
    0x06, 0x98, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04,
    0x98, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x99, 0x01,
    0x04, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x99, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01, 0x14, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x25, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x08, 0x12, 0x04, 0x99, 0x01, 0x27, 0x45, 0x0a, 0x10, 0x0a,
    0x08, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x99, 0x01, 0x28, 0x44, 0x0a,
    0x11, 0x0a, 0x09, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x99, 0x01,
    0x28, 0x3c, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x04, 0x99, 0x01, 0x28, 0x3c, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01, 0x29, 0x3b, 0x0a, 0x11, 0x0a, 0x09, 0x04,
    0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x3f, 0x44, 0x0a, 0x39,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x46, 0x1a, 0x2b, 0x20, 0x49,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x2c, 0x20, 0x64, 0x6f, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x9b, 0x01, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x9b, 0x01, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x08, 0x12, 0x04,
    0x9b, 0x01, 0x27, 0x45, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x15, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x04, 0x9b, 0x01, 0x28, 0x44, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x15, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x04, 0x9b, 0x01, 0x28, 0x3c, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x15, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x28, 0x3c, 0x0a, 0x13, 0x0a,
    0x0b, 0x04, 0x15, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x01,
    0x29, 0x3b, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x15, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x04, 0x9b, 0x01, 0x3f, 0x44, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x9e, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17,
    0x12, 0x06, 0xa2, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12,
    0x04, 0xa2, 0x01, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xa3,
    0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa3, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x14, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x21, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x08, 0x12, 0x04, 0xa3, 0x01, 0x23, 0x41, 0x0a, 0x10,
    0x0a, 0x08, 0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xa3, 0x01, 0x24, 0x40,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xa3,
    0x01, 0x24, 0x38, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xa3, 0x01, 0x24, 0x38, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x17, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x25, 0x37, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x3b, 0x40, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xa6, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x00, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xa7, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa7, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa7, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xaa, 0x01, 0x00, 0xac,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xab, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12,
    0x06, 0xae, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04,
    0xae, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xaf, 0x01,
    0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x14, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06,
    0xb3, 0x01, 0x00, 0xb6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xb3,
    0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x01, 0x04,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb4, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x13, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb4, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xb5, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x13, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xb5, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xb8,
    0x01, 0x00, 0xba, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xb8, 0x01,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb9, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1d, 0x12, 0x06, 0xbc, 0x01, 0x00, 0xbe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d,
    0x01, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12,
    0x04, 0xbd, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xbd, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbd,
    0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x01,
    0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x19,
    0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xc0, 0x01, 0x00, 0xc2, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc1, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc1, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xc4, 0x01,
    0x00, 0xd5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x04, 0x61, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc5, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x27, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x40, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x08, 0x12, 0x04, 0xc5, 0x01, 0x42, 0x60, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x43, 0x5f, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xc5, 0x01, 0x43, 0x57, 0x0a,
    0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc5,
    0x01, 0x43, 0x57, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x44, 0x56, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x5a, 0x5f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1f, 0x02, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xc6, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc6, 0x01, 0x27, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xc6, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04, 0xc7,
    0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc7, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x06, 0x12, 0x04, 0xc7, 0x01, 0x0d,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x27, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x40, 0x41, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x03, 0x06, 0x12, 0x04, 0xc8, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x27, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02,
    0x04, 0x12, 0x04, 0xc9, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xc9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x06, 0x12,
    0x04, 0xc9, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xc9, 0x01, 0x27, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc9,
    0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x05, 0x12, 0x04, 0xca, 0x01, 0x04,
    0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x04, 0x12, 0x04, 0xca, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x06, 0x12, 0x04, 0xca, 0x01, 0x0d, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x01, 0x12, 0x04, 0xca, 0x01, 0x27, 0x35, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x03, 0x12, 0x04, 0xca, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x06, 0x12, 0x04, 0xcb, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x06, 0x04, 0x12, 0x04, 0xcb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x06, 0x06, 0x12, 0x04, 0xcb, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x06, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x27, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x06,
    0x03, 0x12, 0x04, 0xcb, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x07, 0x12,
    0x04, 0xcc, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x04, 0x12, 0x04,
    0xcc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x06, 0x12, 0x04, 0xcc,
    0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x01, 0x12, 0x04, 0xcc, 0x01,
    0x27, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x40,
    0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x08, 0x12, 0x04, 0xcd, 0x01, 0x04, 0x43, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x06, 0x12, 0x04, 0xcd, 0x01, 0x0d, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x08, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x27, 0x3d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x08, 0x03, 0x12, 0x04, 0xcd, 0x01, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1f, 0x02, 0x09, 0x12, 0x04, 0xce, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x09, 0x04, 0x12, 0x04, 0xce, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x09,
    0x06, 0x12, 0x04, 0xce, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x09, 0x01,
    0x12, 0x04, 0xce, 0x01, 0x27, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x09, 0x03, 0x12,
    0x04, 0xce, 0x01, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x0a, 0x12, 0x04, 0xcf,
    0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xcf, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xcf, 0x01, 0x0d,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x27, 0x3b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x40, 0x42, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x0b, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xd0, 0x01, 0x0d, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x27, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x0b, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02,
    0x0c, 0x12, 0x04, 0xd2, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0c, 0x04,
    0x12, 0x04, 0xd2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0c, 0x06, 0x12,
    0x04, 0xd2, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0c, 0x01, 0x12, 0x04,
    0xd2, 0x01, 0x27, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xd2,
    0x01, 0x40, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x0d, 0x12, 0x04, 0xd3, 0x01, 0x04,
    0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0d, 0x06, 0x12, 0x04, 0xd3, 0x01, 0x0d, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x27, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x40, 0x43, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x0e, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x0e, 0x06, 0x12, 0x04, 0xd4, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x0e, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x27, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x0e,
    0x03, 0x12, 0x04, 0xd4, 0x01, 0x40, 0x43, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xd7,
    0x01, 0x00, 0xe8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xd7, 0x01,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x63,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x28, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x42, 0x43, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x00, 0x08, 0x12, 0x04, 0xd8, 0x01, 0x44, 0x62, 0x0a, 0x10, 0x0a, 0x08, 0x04,
    0x20, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xd8, 0x01, 0x45, 0x61, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x20, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xd8, 0x01, 0x45, 0x59,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x20, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04,
    0xd8, 0x01, 0x45, 0x59, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x20, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x46, 0x58, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x20, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x5c, 0x61, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xd9, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xd9, 0x01, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xd9, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04,
    0xda, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0xda,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x06, 0x12, 0x04, 0xda, 0x01,
    0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0xda, 0x01, 0x28,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x03, 0x12, 0x04, 0xda, 0x01, 0x42, 0x43,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x44, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x03, 0x06, 0x12, 0x04, 0xdb, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x28, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x03, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20,
    0x02, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x06,
    0x12, 0x04, 0xdc, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xdc, 0x01, 0x28, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xdc, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x05, 0x12, 0x04, 0xdd, 0x01,
    0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x06, 0x12, 0x04, 0xdd, 0x01, 0x0d, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x28, 0x37, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x42, 0x43, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x06, 0x12, 0x04, 0xde, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x06, 0x04, 0x12, 0x04, 0xde, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x06, 0x06, 0x12, 0x04, 0xde, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xde, 0x01, 0x28, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x06, 0x03, 0x12, 0x04, 0xde, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x07,
    0x12, 0x04, 0xdf, 0x01, 0x04, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x04, 0x12,
    0x04, 0xdf, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x06, 0x12, 0x04,
    0xdf, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x01, 0x12, 0x04, 0xdf,
    0x01, 0x28, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x03, 0x12, 0x04, 0xdf, 0x01,
    0x42, 0x44, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x08, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x45,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x08, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x08, 0x06, 0x12, 0x04, 0xe0, 0x01, 0x0d, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x08, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x28, 0x3f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x08, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x42, 0x44, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x09, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x09, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x09, 0x06, 0x12, 0x04, 0xe1, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09,
    0x01, 0x12, 0x04, 0xe1, 0x01, 0x28, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09, 0x03,
    0x12, 0x04, 0xe1, 0x01, 0x42, 0x44, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x0a, 0x12, 0x04,
    0xe2, 0x01, 0x04, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xe2,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xe2, 0x01,
    0x0d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x28,
    0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x42, 0x44,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x0b, 0x12, 0x04, 0xe3, 0x01, 0x04, 0x45, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xe3, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x28, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x42, 0x44, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20,
    0x02, 0x0c, 0x12, 0x04, 0xe5, 0x01, 0x04, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0c,
    0x04, 0x12, 0x04, 0xe5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0c, 0x06,
    0x12, 0x04, 0xe5, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0xe5, 0x01, 0x28, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0xe5, 0x01, 0x42, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x0d, 0x12, 0x04, 0xe6, 0x01,
    0x04, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0d, 0x06, 0x12, 0x04, 0xe6, 0x01, 0x0d, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x28, 0x38, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x42, 0x45, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x0e, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x46, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x0e, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x0e, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x28, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x0e, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x42, 0x45,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
