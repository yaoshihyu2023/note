// This file is generated by rust-protobuf 3.2.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/rpc/status.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobufv3::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:google.rpc.Status)
pub struct Status {
    // message fields
    // @@protoc_insertion_point(field:google.rpc.Status.code)
    pub code: i32,
    // @@protoc_insertion_point(field:google.rpc.Status.message)
    pub message: ::std::string::String,
    // @@protoc_insertion_point(field:google.rpc.Status.details)
    pub details: ::std::vec::Vec<::protobufv3::well_known_types::any::Any>,
    // special fields
    // @@protoc_insertion_point(special_field:google.rpc.Status.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Status {
    fn default() -> &'a Status {
        <Status as ::protobufv3::Message>::default_instance()
    }
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "code",
            |m: &Status| { &m.code },
            |m: &mut Status| { &mut m.code },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &Status| { &m.message },
            |m: &mut Status| { &mut m.message },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "details",
            |m: &Status| { &m.details },
            |m: &mut Status| { &mut m.details },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<Status>(
            "Status",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for Status {
    const NAME: &'static str = "Status";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.code = is.read_int32()?;
                },
                18 => {
                    self.message = is.read_string()?;
                },
                26 => {
                    self.details.push(is.read_message()?);
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.code != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.code);
        }
        if !self.message.is_empty() {
            my_size += ::protobufv3::rt::string_size(2, &self.message);
        }
        for value in &self.details {
            let len = value.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.code != 0 {
            os.write_int32(1, self.code)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        for v in &self.details {
            ::protobufv3::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Status {
        Status::new()
    }

    fn clear(&mut self) {
        self.code = 0;
        self.message.clear();
        self.details.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Status {
        static instance: Status = Status {
            code: 0,
            message: ::std::string::String::new(),
            details: ::std::vec::Vec::new(),
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for Status {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Status").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for Status {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/rpc/status.proto\x12\ngoogle.rpc\x1a\x19google/protobuf/any\
    .proto\"f\n\x06Status\x12\x12\n\x04code\x18\x01\x20\x01(\x05R\x04code\
    \x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\x12.\n\x07details\
    \x18\x03\x20\x03(\x0b2\x14.google.protobuf.AnyR\x07detailsB^\n\x0ecom.go\
    ogle.rpcB\x0bStatusProtoP\x01Z7google.golang.org/genproto/googleapis/rpc\
    /status;status\xa2\x02\x03RPCb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobufv3::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobufv3::rt::Lazy<::protobufv3::descriptor::FileDescriptorProto> = ::protobufv3::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobufv3::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobufv3::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobufv3::rt::Lazy<::protobufv3::reflect::GeneratedFileDescriptor> = ::protobufv3::rt::Lazy::new();
    static file_descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::FileDescriptor> = ::protobufv3::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobufv3::well_known_types::any::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Status::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobufv3::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobufv3::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
