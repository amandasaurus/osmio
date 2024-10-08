// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `fileformat.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_2;

#[derive(PartialEq,Clone,Default)]
pub struct BlobHeader {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    indexdata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    datasize: ::std::option::Option<i32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlobHeader {
    fn default() -> &'a BlobHeader {
        <BlobHeader as ::protobuf::Message>::default_instance()
    }
}

impl BlobHeader {
    pub fn new() -> BlobHeader {
        ::std::default::Default::default()
    }

    // required string type = 1;


    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes indexdata = 2;


    pub fn get_indexdata(&self) -> &[u8] {
        match self.indexdata.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_indexdata(&mut self) {
        self.indexdata.clear();
    }

    pub fn has_indexdata(&self) -> bool {
        self.indexdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indexdata(&mut self, v: ::std::vec::Vec<u8>) {
        self.indexdata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indexdata(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.indexdata.is_none() {
            self.indexdata.set_default();
        }
        self.indexdata.as_mut().unwrap()
    }

    // Take field
    pub fn take_indexdata(&mut self) -> ::std::vec::Vec<u8> {
        self.indexdata.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // required int32 datasize = 3;


    pub fn get_datasize(&self) -> i32 {
        self.datasize.unwrap_or(0)
    }
    pub fn clear_datasize(&mut self) {
        self.datasize = ::std::option::Option::None;
    }

    pub fn has_datasize(&self) -> bool {
        self.datasize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datasize(&mut self, v: i32) {
        self.datasize = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for BlobHeader {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.datasize.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.indexdata)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.datasize = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.indexdata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.datasize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.indexdata.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.datasize {
            os.write_int32(3, v)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BlobHeader {
        BlobHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    |m: &BlobHeader| { &m.field_type },
                    |m: &mut BlobHeader| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "indexdata",
                    |m: &BlobHeader| { &m.indexdata },
                    |m: &mut BlobHeader| { &mut m.indexdata },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "datasize",
                    |m: &BlobHeader| { &m.datasize },
                    |m: &mut BlobHeader| { &mut m.datasize },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlobHeader>(
                    "BlobHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlobHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlobHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlobHeader,
        };
        unsafe {
            instance.get(BlobHeader::new)
        }
    }
}

impl ::protobuf::Clear for BlobHeader {
    fn clear(&mut self) {
        self.field_type.clear();
        self.indexdata.clear();
        self.datasize = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlobHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlobHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Blob {
    // message fields
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    raw_size: ::std::option::Option<i32>,
    zlib_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lzma_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    OBSOLETE_bzip2_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Blob {
    fn default() -> &'a Blob {
        <Blob as ::protobuf::Message>::default_instance()
    }
}

impl Blob {
    pub fn new() -> Blob {
        ::std::default::Default::default()
    }

    // optional bytes raw = 1;


    pub fn get_raw(&self) -> &[u8] {
        match self.raw.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_raw(&mut self) {
        self.raw.clear();
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.raw.is_none() {
            self.raw.set_default();
        }
        self.raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw(&mut self) -> ::std::vec::Vec<u8> {
        self.raw.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional int32 raw_size = 2;


    pub fn get_raw_size(&self) -> i32 {
        self.raw_size.unwrap_or(0)
    }
    pub fn clear_raw_size(&mut self) {
        self.raw_size = ::std::option::Option::None;
    }

    pub fn has_raw_size(&self) -> bool {
        self.raw_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_size(&mut self, v: i32) {
        self.raw_size = ::std::option::Option::Some(v);
    }

    // optional bytes zlib_data = 3;


    pub fn get_zlib_data(&self) -> &[u8] {
        match self.zlib_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_zlib_data(&mut self) {
        self.zlib_data.clear();
    }

    pub fn has_zlib_data(&self) -> bool {
        self.zlib_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zlib_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.zlib_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zlib_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.zlib_data.is_none() {
            self.zlib_data.set_default();
        }
        self.zlib_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_zlib_data(&mut self) -> ::std::vec::Vec<u8> {
        self.zlib_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes lzma_data = 4;


    pub fn get_lzma_data(&self) -> &[u8] {
        match self.lzma_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_lzma_data(&mut self) {
        self.lzma_data.clear();
    }

    pub fn has_lzma_data(&self) -> bool {
        self.lzma_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lzma_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.lzma_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lzma_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.lzma_data.is_none() {
            self.lzma_data.set_default();
        }
        self.lzma_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_lzma_data(&mut self) -> ::std::vec::Vec<u8> {
        self.lzma_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes OBSOLETE_bzip2_data = 5;


    pub fn get_OBSOLETE_bzip2_data(&self) -> &[u8] {
        match self.OBSOLETE_bzip2_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_OBSOLETE_bzip2_data(&mut self) {
        self.OBSOLETE_bzip2_data.clear();
    }

    pub fn has_OBSOLETE_bzip2_data(&self) -> bool {
        self.OBSOLETE_bzip2_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_OBSOLETE_bzip2_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.OBSOLETE_bzip2_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_OBSOLETE_bzip2_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.OBSOLETE_bzip2_data.is_none() {
            self.OBSOLETE_bzip2_data.set_default();
        }
        self.OBSOLETE_bzip2_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_OBSOLETE_bzip2_data(&mut self) -> ::std::vec::Vec<u8> {
        self.OBSOLETE_bzip2_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Blob {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.raw)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.raw_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.zlib_data)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lzma_data)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.OBSOLETE_bzip2_data)?;
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
        if let Some(ref v) = self.raw.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.raw_size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.zlib_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.lzma_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.OBSOLETE_bzip2_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.raw.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.raw_size {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.zlib_data.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.lzma_data.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.OBSOLETE_bzip2_data.as_ref() {
            os.write_bytes(5, &v)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Blob {
        Blob::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "raw",
                    |m: &Blob| { &m.raw },
                    |m: &mut Blob| { &mut m.raw },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "raw_size",
                    |m: &Blob| { &m.raw_size },
                    |m: &mut Blob| { &mut m.raw_size },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "zlib_data",
                    |m: &Blob| { &m.zlib_data },
                    |m: &mut Blob| { &mut m.zlib_data },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "lzma_data",
                    |m: &Blob| { &m.lzma_data },
                    |m: &mut Blob| { &mut m.lzma_data },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "OBSOLETE_bzip2_data",
                    |m: &Blob| { &m.OBSOLETE_bzip2_data },
                    |m: &mut Blob| { &mut m.OBSOLETE_bzip2_data },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Blob>(
                    "Blob",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Blob {
        static mut instance: ::protobuf::lazy::Lazy<Blob> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Blob,
        };
        unsafe {
            instance.get(Blob::new)
        }
    }
}

impl ::protobuf::Clear for Blob {
    fn clear(&mut self) {
        self.raw.clear();
        self.raw_size = ::std::option::Option::None;
        self.zlib_data.clear();
        self.lzma_data.clear();
        self.OBSOLETE_bzip2_data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Blob {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10fileformat.proto\"Z\n\nBlobHeader\x12\x12\n\x04type\x18\x01\x20\
    \x02(\tR\x04type\x12\x1c\n\tindexdata\x18\x02\x20\x01(\x0cR\tindexdata\
    \x12\x1a\n\x08datasize\x18\x03\x20\x02(\x05R\x08datasize\"\xa1\x01\n\x04\
    Blob\x12\x10\n\x03raw\x18\x01\x20\x01(\x0cR\x03raw\x12\x19\n\x08raw_size\
    \x18\x02\x20\x01(\x05R\x07rawSize\x12\x1b\n\tzlib_data\x18\x03\x20\x01(\
    \x0cR\x08zlibData\x12\x1b\n\tlzma_data\x18\x04\x20\x01(\x0cR\x08lzmaData\
    \x122\n\x13OBSOLETE_bzip2_data\x18\x05\x20\x01(\x0cR\x11oBSOLETEBzip2Dat\
    aB\x02\x18\x01J\xdb\x07\n\x06\x12\x04\0\0\x12\x01\n\n\n\x02\x04\0\x12\
    \x04\0\0\x04\x01\n\n\n\x03\x04\0\x01\x12\x03\0\x08\x12\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x01\x04\x1d\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x01\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x01\r\x13\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x01\x14\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x01\x1b\
    \x1c\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x02\x04!\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x03\x02\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x02\r\
    \x12\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x02\x13\x1c\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x02\x1f\x20\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x03\
    \x04\x20\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x03\r\x12\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x03\x13\x1b\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x03\x1e\x1f\n\n\n\
    \x02\x04\x01\x12\x04\x06\0\x12\x01\n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\
    \x0c\n\x1d\n\x04\x04\x01\x02\0\x12\x03\x07\x04\x1b\"\x10\x20No\x20compre\
    ssion\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x07\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x07\r\x12\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x07\x13\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x07\x19\x1a\n5\n\
    \x04\x04\x01\x02\x01\x12\x03\x08\x04\x20\"(\x20When\x20compressed,\x20th\
    e\x20uncompressed\x20size\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x08\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x08\r\x12\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03\x08\x13\x1b\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03\x08\x1e\x1f\n8\n\x04\x04\x01\x02\x02\x12\x03\x0b\x04!\x1a+\x20P\
    ossible\x20compressed\x20versions\x20of\x20the\x20data.\n\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x0b\r\x12\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0b\x13\x1c\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0b\x1f\x20\nR\n\x04\x04\x01\x02\
    \x03\x12\x03\x0e\x04!\x1aE\x20PROPOSED\x20feature\x20for\x20LZMA\x20comp\
    ressed\x20data.\x20SUPPORT\x20IS\x20NOT\x20REQUIRED.\n\n\x0c\n\x05\x04\
    \x01\x02\x03\x04\x12\x03\x0e\x04\x0c\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\
    \x03\x0e\r\x12\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0e\x13\x1c\n\x0c\
    \n\x05\x04\x01\x02\x03\x03\x12\x03\x0e\x1f\x20\nl\n\x04\x04\x01\x02\x04\
    \x12\x03\x11\x04=\x1a?\x20Formerly\x20used\x20for\x20bzip2\x20compressed\
    \x20data.\x20Depreciated\x20in\x202010.\n\"\x1e\x20Don't\x20reuse\x20thi\
    s\x20tag\x20number.\n\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03\x11\x04\
    \x0c\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x11\r\x12\n\x0c\n\x05\x04\
    \x01\x02\x04\x01\x12\x03\x11\x13&\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\
    \x03\x11)*\n\x0c\n\x05\x04\x01\x02\x04\x08\x12\x03\x11+<\n\x0f\n\x08\x04\
    \x01\x02\x04\x08\xe7\x07\0\x12\x03\x11,;\n\x10\n\t\x04\x01\x02\x04\x08\
    \xe7\x07\0\x02\x12\x03\x11,6\n\x11\n\n\x04\x01\x02\x04\x08\xe7\x07\0\x02\
    \0\x12\x03\x11,6\n\x12\n\x0b\x04\x01\x02\x04\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x11,6\n\x10\n\t\x04\x01\x02\x04\x08\xe7\x07\0\x03\x12\x03\x117;\
";

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
