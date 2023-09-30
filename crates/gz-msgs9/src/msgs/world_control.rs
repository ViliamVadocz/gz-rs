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

//! Generated file from `gz/msgs/world_control.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.WorldControl)
pub struct WorldControl {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.pause)
    pub pause: bool,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.step)
    pub step: bool,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.multi_step)
    pub multi_step: u32,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.reset)
    pub reset: ::protobuf::MessageField<super::world_reset::WorldReset>,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.seed)
    pub seed: u32,
    // @@protoc_insertion_point(field:gz.msgs.WorldControl.run_to_sim_time)
    pub run_to_sim_time: ::protobuf::MessageField<super::time::Time>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.WorldControl.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WorldControl {
    fn default() -> &'a WorldControl {
        <WorldControl as ::protobuf::Message>::default_instance()
    }
}

impl WorldControl {
    pub fn new() -> WorldControl {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &WorldControl| { &m.header },
            |m: &mut WorldControl| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pause",
            |m: &WorldControl| { &m.pause },
            |m: &mut WorldControl| { &mut m.pause },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &WorldControl| { &m.step },
            |m: &mut WorldControl| { &mut m.step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "multi_step",
            |m: &WorldControl| { &m.multi_step },
            |m: &mut WorldControl| { &mut m.multi_step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::world_reset::WorldReset>(
            "reset",
            |m: &WorldControl| { &m.reset },
            |m: &mut WorldControl| { &mut m.reset },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "seed",
            |m: &WorldControl| { &m.seed },
            |m: &mut WorldControl| { &mut m.seed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "run_to_sim_time",
            |m: &WorldControl| { &m.run_to_sim_time },
            |m: &mut WorldControl| { &mut m.run_to_sim_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WorldControl>(
            "WorldControl",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WorldControl {
    const NAME: &'static str = "WorldControl";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                16 => {
                    self.pause = is.read_bool()?;
                },
                24 => {
                    self.step = is.read_bool()?;
                },
                32 => {
                    self.multi_step = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reset)?;
                },
                48 => {
                    self.seed = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.run_to_sim_time)?;
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.pause != false {
            my_size += 1 + 1;
        }
        if self.step != false {
            my_size += 1 + 1;
        }
        if self.multi_step != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.multi_step);
        }
        if let Some(v) = self.reset.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.seed != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.seed);
        }
        if let Some(v) = self.run_to_sim_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.pause != false {
            os.write_bool(2, self.pause)?;
        }
        if self.step != false {
            os.write_bool(3, self.step)?;
        }
        if self.multi_step != 0 {
            os.write_uint32(4, self.multi_step)?;
        }
        if let Some(v) = self.reset.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.seed != 0 {
            os.write_uint32(6, self.seed)?;
        }
        if let Some(v) = self.run_to_sim_time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> WorldControl {
        WorldControl::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.pause = false;
        self.step = false;
        self.multi_step = 0;
        self.reset.clear();
        self.seed = 0;
        self.run_to_sim_time.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WorldControl {
        static instance: WorldControl = WorldControl {
            header: ::protobuf::MessageField::none(),
            pause: false,
            step: false,
            multi_step: 0,
            reset: ::protobuf::MessageField::none(),
            seed: 0,
            run_to_sim_time: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WorldControl {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WorldControl").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WorldControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorldControl {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgz/msgs/world_control.proto\x12\x07gz.msgs\x1a\x19gz/msgs/world_re\
    set.proto\x1a\x14gz/msgs/header.proto\x1a\x12gz/msgs/time.proto\"\xf5\
    \x01\n\x0cWorldControl\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msg\
    s.HeaderR\x06header\x12\x14\n\x05pause\x18\x02\x20\x01(\x08R\x05pause\
    \x12\x12\n\x04step\x18\x03\x20\x01(\x08R\x04step\x12\x1d\n\nmulti_step\
    \x18\x04\x20\x01(\rR\tmultiStep\x12)\n\x05reset\x18\x05\x20\x01(\x0b2\
    \x13.gz.msgs.WorldResetR\x05reset\x12\x12\n\x04seed\x18\x06\x20\x01(\rR\
    \x04seed\x124\n\x0frun_to_sim_time\x18\x07\x20\x01(\x0b2\r.gz.msgs.TimeR\
    \x0crunToSimTimeB!\n\x0bcom.gz.msgsB\x12WorldControlProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::world_reset::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::time::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(WorldControl::generated_message_descriptor_data());
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
