// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]
#![allow(warnings)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct BlobHeader {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    indexdata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    datasize: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlobHeader {}

impl BlobHeader {
    pub fn new() -> BlobHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlobHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlobHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlobHeader,
        };
        unsafe {
            instance.get(|| {
                BlobHeader {
                    field_type: ::protobuf::SingularField::none(),
                    indexdata: ::protobuf::SingularField::none(),
                    datasize: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string type = 1;

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
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes indexdata = 2;

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
        };
        self.indexdata.as_mut().unwrap()
    }

    // Take field
    pub fn take_indexdata(&mut self) -> ::std::vec::Vec<u8> {
        self.indexdata.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_indexdata(&self) -> &[u8] {
        match self.indexdata.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required int32 datasize = 3;

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

    pub fn get_datasize(&self) -> i32 {
        self.datasize.unwrap_or(0)
    }
}

impl ::protobuf::Message for BlobHeader {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.datasize.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.indexdata));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.datasize = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.indexdata.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.datasize.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.indexdata.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.datasize {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlobHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlobHeader {
    fn new() -> BlobHeader {
        BlobHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlobHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "type",
                    BlobHeader::has_field_type,
                    BlobHeader::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "indexdata",
                    BlobHeader::has_indexdata,
                    BlobHeader::get_indexdata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "datasize",
                    BlobHeader::has_datasize,
                    BlobHeader::get_datasize,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlobHeader>(
                    "BlobHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlobHeader {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_indexdata();
        self.clear_datasize();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlobHeader {
    fn eq(&self, other: &BlobHeader) -> bool {
        self.field_type == other.field_type &&
        self.indexdata == other.indexdata &&
        self.datasize == other.datasize &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlobHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Blob {
    // message fields
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    raw_size: ::std::option::Option<i32>,
    zlib_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lzma_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    OBSOLETE_bzip2_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Blob {}

impl Blob {
    pub fn new() -> Blob {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Blob {
        static mut instance: ::protobuf::lazy::Lazy<Blob> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Blob,
        };
        unsafe {
            instance.get(|| {
                Blob {
                    raw: ::protobuf::SingularField::none(),
                    raw_size: ::std::option::Option::None,
                    zlib_data: ::protobuf::SingularField::none(),
                    lzma_data: ::protobuf::SingularField::none(),
                    OBSOLETE_bzip2_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes raw = 1;

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
        };
        self.raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw(&mut self) -> ::std::vec::Vec<u8> {
        self.raw.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_raw(&self) -> &[u8] {
        match self.raw.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional int32 raw_size = 2;

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

    pub fn get_raw_size(&self) -> i32 {
        self.raw_size.unwrap_or(0)
    }

    // optional bytes zlib_data = 3;

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
        };
        self.zlib_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_zlib_data(&mut self) -> ::std::vec::Vec<u8> {
        self.zlib_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_zlib_data(&self) -> &[u8] {
        match self.zlib_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes lzma_data = 4;

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
        };
        self.lzma_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_lzma_data(&mut self) -> ::std::vec::Vec<u8> {
        self.lzma_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_lzma_data(&self) -> &[u8] {
        match self.lzma_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes OBSOLETE_bzip2_data = 5;

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
        };
        self.OBSOLETE_bzip2_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_OBSOLETE_bzip2_data(&mut self) -> ::std::vec::Vec<u8> {
        self.OBSOLETE_bzip2_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_OBSOLETE_bzip2_data(&self) -> &[u8] {
        match self.OBSOLETE_bzip2_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Blob {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.raw));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.raw_size = ::std::option::Option::Some(tmp);
                },
                3 => {
                        try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.zlib_data));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lzma_data));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.OBSOLETE_bzip2_data));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.raw.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.raw_size.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.zlib_data.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.lzma_data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.OBSOLETE_bzip2_data.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.raw.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.raw_size {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.zlib_data.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.lzma_data.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.OBSOLETE_bzip2_data.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Blob>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Blob {
    fn new() -> Blob {
        Blob::new()
    }

    fn descriptor_static(_: ::std::option::Option<Blob>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "raw",
                    Blob::has_raw,
                    Blob::get_raw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "raw_size",
                    Blob::has_raw_size,
                    Blob::get_raw_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "zlib_data",
                    Blob::has_zlib_data,
                    Blob::get_zlib_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "lzma_data",
                    Blob::has_lzma_data,
                    Blob::get_lzma_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "OBSOLETE_bzip2_data",
                    Blob::has_OBSOLETE_bzip2_data,
                    Blob::get_OBSOLETE_bzip2_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Blob>(
                    "Blob",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Blob {
    fn clear(&mut self) {
        self.clear_raw();
        self.clear_raw_size();
        self.clear_zlib_data();
        self.clear_lzma_data();
        self.clear_OBSOLETE_bzip2_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Blob {
    fn eq(&self, other: &Blob) -> bool {
        self.raw == other.raw &&
        self.raw_size == other.raw_size &&
        self.zlib_data == other.zlib_data &&
        self.lzma_data == other.lzma_data &&
        self.OBSOLETE_bzip2_data == other.OBSOLETE_bzip2_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x66, 0x69, 0x6c, 0x65, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x3f, 0x0a, 0x0a, 0x42, 0x6c, 0x6f, 0x62, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11,
    0x0a, 0x09, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x61, 0x74, 0x61, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x05, 0x22, 0x6c, 0x0a, 0x04, 0x42, 0x6c, 0x6f, 0x62, 0x12, 0x0b, 0x0a, 0x03, 0x72,
    0x61, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x61, 0x77, 0x5f,
    0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x7a, 0x6c,
    0x69, 0x62, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a,
    0x09, 0x6c, 0x7a, 0x6d, 0x61, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x1f, 0x0a, 0x13, 0x4f, 0x42, 0x53, 0x4f, 0x4c, 0x45, 0x54, 0x45, 0x5f, 0x62, 0x7a, 0x69,
    0x70, 0x32, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x02, 0x18,
    0x01, 0x4a, 0xdb, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x00, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x02, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x02, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x02,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x13, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x02, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x03, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x03, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x06, 0x00, 0x12, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x06, 0x08, 0x0c, 0x0a, 0x1d, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x1b, 0x22, 0x10, 0x20, 0x4e, 0x6f, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x07, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07,
    0x19, 0x1a, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x20, 0x22,
    0x28, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x64, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x08, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x08, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1e,
    0x1f, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x04, 0x21, 0x1a, 0x2b,
    0x20, 0x50, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0b, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0b, 0x1f, 0x20, 0x0a, 0x52, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x04, 0x21,
    0x1a, 0x45, 0x20, 0x50, 0x52, 0x4f, 0x50, 0x4f, 0x53, 0x45, 0x44, 0x20, 0x66, 0x65, 0x61, 0x74,
    0x75, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x4c, 0x5a, 0x4d, 0x41, 0x20, 0x63, 0x6f, 0x6d,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x53, 0x55,
    0x50, 0x50, 0x4f, 0x52, 0x54, 0x20, 0x49, 0x53, 0x20, 0x4e, 0x4f, 0x54, 0x20, 0x52, 0x45, 0x51,
    0x55, 0x49, 0x52, 0x45, 0x44, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x0e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x13,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x1f, 0x20, 0x0a,
    0x6c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x3d, 0x1a, 0x3f, 0x20, 0x46,
    0x6f, 0x72, 0x6d, 0x65, 0x72, 0x6c, 0x79, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x62, 0x7a, 0x69, 0x70, 0x32, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x69, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x32, 0x30, 0x31, 0x30, 0x2e, 0x0a, 0x22, 0x1e, 0x20,
    0x44, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x72, 0x65, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x74, 0x61, 0x67, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x11, 0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x11, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x08, 0x12, 0x03,
    0x11, 0x2b, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x11, 0x2c, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x11, 0x2c, 0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x2c, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x2c, 0x36, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x11, 0x37, 0x3b,
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
