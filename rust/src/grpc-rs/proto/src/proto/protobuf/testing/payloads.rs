// This file is generated by rust-protobuf 2.8.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `grpc/testing/payloads.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.

#[derive(PartialEq,Clone,Default)]
pub struct ByteBufferParams {
    // message fields
    pub req_size: i32,
    pub resp_size: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ByteBufferParams {
    fn default() -> &'a ByteBufferParams {
        <ByteBufferParams as ::protobuf::Message>::default_instance()
    }
}

impl ByteBufferParams {
    pub fn new() -> ByteBufferParams {
        ::std::default::Default::default()
    }

    // int32 req_size = 1;


    pub fn get_req_size(&self) -> i32 {
        self.req_size
    }
    pub fn clear_req_size(&mut self) {
        self.req_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_req_size(&mut self, v: i32) {
        self.req_size = v;
    }

    // int32 resp_size = 2;


    pub fn get_resp_size(&self) -> i32 {
        self.resp_size
    }
    pub fn clear_resp_size(&mut self) {
        self.resp_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_resp_size(&mut self, v: i32) {
        self.resp_size = v;
    }
}

impl ::protobuf::Message for ByteBufferParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.req_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.resp_size = tmp;
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
        if self.req_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.req_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resp_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.resp_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
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

    fn new() -> ByteBufferParams {
        ByteBufferParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "req_size",
                    |m: &ByteBufferParams| { &m.req_size },
                    |m: &mut ByteBufferParams| { &mut m.req_size },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "resp_size",
                    |m: &ByteBufferParams| { &m.resp_size },
                    |m: &mut ByteBufferParams| { &mut m.resp_size },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ByteBufferParams>(
                    "ByteBufferParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ByteBufferParams {
        static mut instance: ::protobuf::lazy::Lazy<ByteBufferParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ByteBufferParams,
        };
        unsafe {
            instance.get(ByteBufferParams::new)
        }
    }
}

impl ::protobuf::Clear for ByteBufferParams {
    fn clear(&mut self) {
        self.req_size = 0;
        self.resp_size = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ByteBufferParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ByteBufferParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SimpleProtoParams {
    // message fields
    pub req_size: i32,
    pub resp_size: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SimpleProtoParams {
    fn default() -> &'a SimpleProtoParams {
        <SimpleProtoParams as ::protobuf::Message>::default_instance()
    }
}

impl SimpleProtoParams {
    pub fn new() -> SimpleProtoParams {
        ::std::default::Default::default()
    }

    // int32 req_size = 1;


    pub fn get_req_size(&self) -> i32 {
        self.req_size
    }
    pub fn clear_req_size(&mut self) {
        self.req_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_req_size(&mut self, v: i32) {
        self.req_size = v;
    }

    // int32 resp_size = 2;


    pub fn get_resp_size(&self) -> i32 {
        self.resp_size
    }
    pub fn clear_resp_size(&mut self) {
        self.resp_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_resp_size(&mut self, v: i32) {
        self.resp_size = v;
    }
}

impl ::protobuf::Message for SimpleProtoParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.req_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.resp_size = tmp;
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
        if self.req_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.req_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resp_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.resp_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
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

    fn new() -> SimpleProtoParams {
        SimpleProtoParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "req_size",
                    |m: &SimpleProtoParams| { &m.req_size },
                    |m: &mut SimpleProtoParams| { &mut m.req_size },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "resp_size",
                    |m: &SimpleProtoParams| { &m.resp_size },
                    |m: &mut SimpleProtoParams| { &mut m.resp_size },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimpleProtoParams>(
                    "SimpleProtoParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SimpleProtoParams {
        static mut instance: ::protobuf::lazy::Lazy<SimpleProtoParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimpleProtoParams,
        };
        unsafe {
            instance.get(SimpleProtoParams::new)
        }
    }
}

impl ::protobuf::Clear for SimpleProtoParams {
    fn clear(&mut self) {
        self.req_size = 0;
        self.resp_size = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SimpleProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleProtoParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ComplexProtoParams {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ComplexProtoParams {
    fn default() -> &'a ComplexProtoParams {
        <ComplexProtoParams as ::protobuf::Message>::default_instance()
    }
}

impl ComplexProtoParams {
    pub fn new() -> ComplexProtoParams {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for ComplexProtoParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
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

    fn new() -> ComplexProtoParams {
        ComplexProtoParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ComplexProtoParams>(
                    "ComplexProtoParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ComplexProtoParams {
        static mut instance: ::protobuf::lazy::Lazy<ComplexProtoParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ComplexProtoParams,
        };
        unsafe {
            instance.get(ComplexProtoParams::new)
        }
    }
}

impl ::protobuf::Clear for ComplexProtoParams {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ComplexProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComplexProtoParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PayloadConfig {
    // message oneof groups
    pub payload: ::std::option::Option<PayloadConfig_oneof_payload>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PayloadConfig {
    fn default() -> &'a PayloadConfig {
        <PayloadConfig as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum PayloadConfig_oneof_payload {
    bytebuf_params(ByteBufferParams),
    simple_params(SimpleProtoParams),
    complex_params(ComplexProtoParams),
}

impl PayloadConfig {
    pub fn new() -> PayloadConfig {
        ::std::default::Default::default()
    }

    // .grpc.testing.ByteBufferParams bytebuf_params = 1;


    pub fn get_bytebuf_params(&self) -> &ByteBufferParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ref v)) => v,
            _ => ByteBufferParams::default_instance(),
        }
    }
    pub fn clear_bytebuf_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_bytebuf_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytebuf_params(&mut self, v: ByteBufferParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytebuf_params(&mut self) -> &mut ByteBufferParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ByteBufferParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytebuf_params(&mut self) -> ByteBufferParams {
        if self.has_bytebuf_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(v)) => v,
                _ => panic!(),
            }
        } else {
            ByteBufferParams::new()
        }
    }

    // .grpc.testing.SimpleProtoParams simple_params = 2;


    pub fn get_simple_params(&self) -> &SimpleProtoParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(ref v)) => v,
            _ => SimpleProtoParams::default_instance(),
        }
    }
    pub fn clear_simple_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_simple_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_simple_params(&mut self, v: SimpleProtoParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_simple_params(&mut self) -> &mut SimpleProtoParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(SimpleProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_simple_params(&mut self) -> SimpleProtoParams {
        if self.has_simple_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(v)) => v,
                _ => panic!(),
            }
        } else {
            SimpleProtoParams::new()
        }
    }

    // .grpc.testing.ComplexProtoParams complex_params = 3;


    pub fn get_complex_params(&self) -> &ComplexProtoParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ref v)) => v,
            _ => ComplexProtoParams::default_instance(),
        }
    }
    pub fn clear_complex_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_complex_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_complex_params(&mut self, v: ComplexProtoParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_complex_params(&mut self) -> &mut ComplexProtoParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ComplexProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_complex_params(&mut self) -> ComplexProtoParams {
        if self.has_complex_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(v)) => v,
                _ => panic!(),
            }
        } else {
            ComplexProtoParams::new()
        }
    }
}

impl ::protobuf::Message for PayloadConfig {
    fn is_initialized(&self) -> bool {
        if let Some(PayloadConfig_oneof_payload::bytebuf_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(PayloadConfig_oneof_payload::simple_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(PayloadConfig_oneof_payload::complex_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &PayloadConfig_oneof_payload::bytebuf_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &PayloadConfig_oneof_payload::simple_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &PayloadConfig_oneof_payload::complex_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &PayloadConfig_oneof_payload::bytebuf_params(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &PayloadConfig_oneof_payload::simple_params(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &PayloadConfig_oneof_payload::complex_params(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn new() -> PayloadConfig {
        PayloadConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ByteBufferParams>(
                    "bytebuf_params",
                    PayloadConfig::has_bytebuf_params,
                    PayloadConfig::get_bytebuf_params,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SimpleProtoParams>(
                    "simple_params",
                    PayloadConfig::has_simple_params,
                    PayloadConfig::get_simple_params,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ComplexProtoParams>(
                    "complex_params",
                    PayloadConfig::has_complex_params,
                    PayloadConfig::get_complex_params,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PayloadConfig>(
                    "PayloadConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PayloadConfig {
        static mut instance: ::protobuf::lazy::Lazy<PayloadConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PayloadConfig,
        };
        unsafe {
            instance.get(PayloadConfig::new)
        }
    }
}

impl ::protobuf::Clear for PayloadConfig {
    fn clear(&mut self) {
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PayloadConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PayloadConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/testing/payloads.proto\x12\x0cgrpc.testing\"J\n\x10ByteBuffer\
    Params\x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\
    \tresp_size\x18\x02\x20\x01(\x05R\x08respSize\"K\n\x11SimpleProtoParams\
    \x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\tresp_\
    size\x18\x02\x20\x01(\x05R\x08respSize\"\x14\n\x12ComplexProtoParams\"\
    \xf6\x01\n\rPayloadConfig\x12G\n\x0ebytebuf_params\x18\x01\x20\x01(\x0b2\
    \x1e.grpc.testing.ByteBufferParamsH\0R\rbytebufParams\x12F\n\rsimple_par\
    ams\x18\x02\x20\x01(\x0b2\x1f.grpc.testing.SimpleProtoParamsH\0R\x0csimp\
    leParams\x12I\n\x0ecomplex_params\x18\x03\x20\x01(\x0b2\x20.grpc.testing\
    .ComplexProtoParamsH\0R\rcomplexParamsB\t\n\x07payloadb\x06proto3\
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
