// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
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

//! Generated file from `gz/msgs/dvl_beam_state.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.DVLBeamState)
pub struct DVLBeamState {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.id)
    pub id: i32,
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.velocity)
    pub velocity: ::protobuf::MessageField<super::dvl_kinematic_estimate::DVLKinematicEstimate>,
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.range)
    pub range: ::protobuf::MessageField<super::dvl_range_estimate::DVLRangeEstimate>,
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.rssi)
    pub rssi: f64,
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.nsd)
    pub nsd: f64,
    // @@protoc_insertion_point(field:gz.msgs.DVLBeamState.locked)
    pub locked: bool,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.DVLBeamState.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DVLBeamState {
    fn default() -> &'a DVLBeamState {
        <DVLBeamState as ::protobuf::Message>::default_instance()
    }
}

impl DVLBeamState {
    pub fn new() -> DVLBeamState {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &DVLBeamState| { &m.id },
            |m: &mut DVLBeamState| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::dvl_kinematic_estimate::DVLKinematicEstimate>(
            "velocity",
            |m: &DVLBeamState| { &m.velocity },
            |m: &mut DVLBeamState| { &mut m.velocity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::dvl_range_estimate::DVLRangeEstimate>(
            "range",
            |m: &DVLBeamState| { &m.range },
            |m: &mut DVLBeamState| { &mut m.range },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rssi",
            |m: &DVLBeamState| { &m.rssi },
            |m: &mut DVLBeamState| { &mut m.rssi },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nsd",
            |m: &DVLBeamState| { &m.nsd },
            |m: &mut DVLBeamState| { &mut m.nsd },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "locked",
            |m: &DVLBeamState| { &m.locked },
            |m: &mut DVLBeamState| { &mut m.locked },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DVLBeamState>(
            "DVLBeamState",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DVLBeamState {
    const NAME: &'static str = "DVLBeamState";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_int32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.velocity)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.range)?;
                },
                33 => {
                    self.rssi = is.read_double()?;
                },
                41 => {
                    self.nsd = is.read_double()?;
                },
                48 => {
                    self.locked = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.id);
        }
        if let Some(v) = self.velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.range.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.rssi != 0. {
            my_size += 1 + 8;
        }
        if self.nsd != 0. {
            my_size += 1 + 8;
        }
        if self.locked != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if let Some(v) = self.velocity.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.range.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.rssi != 0. {
            os.write_double(4, self.rssi)?;
        }
        if self.nsd != 0. {
            os.write_double(5, self.nsd)?;
        }
        if self.locked != false {
            os.write_bool(6, self.locked)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DVLBeamState {
        DVLBeamState::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.velocity.clear();
        self.range.clear();
        self.rssi = 0.;
        self.nsd = 0.;
        self.locked = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DVLBeamState {
        static instance: DVLBeamState = DVLBeamState {
            id: 0,
            velocity: ::protobuf::MessageField::none(),
            range: ::protobuf::MessageField::none(),
            rssi: 0.,
            nsd: 0.,
            locked: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DVLBeamState {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DVLBeamState").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DVLBeamState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DVLBeamState {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgz/msgs/dvl_beam_state.proto\x12\x07gz.msgs\x1a$gz/msgs/dvl_kinema\
    tic_estimate.proto\x1a\x20gz/msgs/dvl_range_estimate.proto\"\xc8\x01\n\
    \x0cDVLBeamState\x12\x0e\n\x02id\x18\x01\x20\x01(\x05R\x02id\x129\n\x08v\
    elocity\x18\x02\x20\x01(\x0b2\x1d.gz.msgs.DVLKinematicEstimateR\x08veloc\
    ity\x12/\n\x05range\x18\x03\x20\x01(\x0b2\x19.gz.msgs.DVLRangeEstimateR\
    \x05range\x12\x12\n\x04rssi\x18\x04\x20\x01(\x01R\x04rssi\x12\x10\n\x03n\
    sd\x18\x05\x20\x01(\x01R\x03nsd\x12\x16\n\x06locked\x18\x06\x20\x01(\x08\
    R\x06lockedB!\n\x0bcom.gz.msgsB\x12DVLBeamStateProtosb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::dvl_kinematic_estimate::file_descriptor().clone());
            deps.push(super::dvl_range_estimate::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DVLBeamState::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
