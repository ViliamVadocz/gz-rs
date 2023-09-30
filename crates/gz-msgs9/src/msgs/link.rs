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

//! Generated file from `gz/msgs/link.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Link)
pub struct Link {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Link.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Link.id)
    pub id: u32,
    // @@protoc_insertion_point(field:gz.msgs.Link.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Link.self_collide)
    pub self_collide: bool,
    // @@protoc_insertion_point(field:gz.msgs.Link.gravity)
    pub gravity: bool,
    // @@protoc_insertion_point(field:gz.msgs.Link.kinematic)
    pub kinematic: bool,
    // @@protoc_insertion_point(field:gz.msgs.Link.enabled)
    pub enabled: bool,
    // @@protoc_insertion_point(field:gz.msgs.Link.density)
    pub density: ::protobuf::MessageField<super::density::Density>,
    // @@protoc_insertion_point(field:gz.msgs.Link.inertial)
    pub inertial: ::protobuf::MessageField<super::inertial::Inertial>,
    // @@protoc_insertion_point(field:gz.msgs.Link.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.Link.visual)
    pub visual: ::std::vec::Vec<super::visual::Visual>,
    // @@protoc_insertion_point(field:gz.msgs.Link.collision)
    pub collision: ::std::vec::Vec<super::collision::Collision>,
    // @@protoc_insertion_point(field:gz.msgs.Link.sensor)
    pub sensor: ::std::vec::Vec<super::sensor::Sensor>,
    // @@protoc_insertion_point(field:gz.msgs.Link.projector)
    pub projector: ::std::vec::Vec<super::projector::Projector>,
    // @@protoc_insertion_point(field:gz.msgs.Link.canonical)
    pub canonical: bool,
    // @@protoc_insertion_point(field:gz.msgs.Link.battery)
    pub battery: ::std::vec::Vec<super::battery::Battery>,
    // @@protoc_insertion_point(field:gz.msgs.Link.light)
    pub light: ::std::vec::Vec<super::light::Light>,
    // @@protoc_insertion_point(field:gz.msgs.Link.particle_emitter)
    pub particle_emitter: ::std::vec::Vec<super::particle_emitter::ParticleEmitter>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Link.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Link {
    fn default() -> &'a Link {
        <Link as ::protobuf::Message>::default_instance()
    }
}

impl Link {
    pub fn new() -> Link {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Link| { &m.header },
            |m: &mut Link| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Link| { &m.id },
            |m: &mut Link| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Link| { &m.name },
            |m: &mut Link| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "self_collide",
            |m: &Link| { &m.self_collide },
            |m: &mut Link| { &mut m.self_collide },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gravity",
            |m: &Link| { &m.gravity },
            |m: &mut Link| { &mut m.gravity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "kinematic",
            |m: &Link| { &m.kinematic },
            |m: &mut Link| { &mut m.kinematic },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "enabled",
            |m: &Link| { &m.enabled },
            |m: &mut Link| { &mut m.enabled },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::density::Density>(
            "density",
            |m: &Link| { &m.density },
            |m: &mut Link| { &mut m.density },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::inertial::Inertial>(
            "inertial",
            |m: &Link| { &m.inertial },
            |m: &mut Link| { &mut m.inertial },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Link| { &m.pose },
            |m: &mut Link| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "visual",
            |m: &Link| { &m.visual },
            |m: &mut Link| { &mut m.visual },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "collision",
            |m: &Link| { &m.collision },
            |m: &mut Link| { &mut m.collision },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sensor",
            |m: &Link| { &m.sensor },
            |m: &mut Link| { &mut m.sensor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "projector",
            |m: &Link| { &m.projector },
            |m: &mut Link| { &mut m.projector },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "canonical",
            |m: &Link| { &m.canonical },
            |m: &mut Link| { &mut m.canonical },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "battery",
            |m: &Link| { &m.battery },
            |m: &mut Link| { &mut m.battery },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "light",
            |m: &Link| { &m.light },
            |m: &mut Link| { &mut m.light },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "particle_emitter",
            |m: &Link| { &m.particle_emitter },
            |m: &mut Link| { &mut m.particle_emitter },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Link>(
            "Link",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Link {
    const NAME: &'static str = "Link";

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
                    self.id = is.read_uint32()?;
                },
                26 => {
                    self.name = is.read_string()?;
                },
                32 => {
                    self.self_collide = is.read_bool()?;
                },
                40 => {
                    self.gravity = is.read_bool()?;
                },
                48 => {
                    self.kinematic = is.read_bool()?;
                },
                56 => {
                    self.enabled = is.read_bool()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.density)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.inertial)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                90 => {
                    self.visual.push(is.read_message()?);
                },
                98 => {
                    self.collision.push(is.read_message()?);
                },
                106 => {
                    self.sensor.push(is.read_message()?);
                },
                114 => {
                    self.projector.push(is.read_message()?);
                },
                120 => {
                    self.canonical = is.read_bool()?;
                },
                130 => {
                    self.battery.push(is.read_message()?);
                },
                138 => {
                    self.light.push(is.read_message()?);
                },
                146 => {
                    self.particle_emitter.push(is.read_message()?);
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if self.self_collide != false {
            my_size += 1 + 1;
        }
        if self.gravity != false {
            my_size += 1 + 1;
        }
        if self.kinematic != false {
            my_size += 1 + 1;
        }
        if self.enabled != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.density.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.inertial.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.visual {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.collision {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.sensor {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.projector {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.canonical != false {
            my_size += 1 + 1;
        }
        for value in &self.battery {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.light {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.particle_emitter {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.id != 0 {
            os.write_uint32(2, self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if self.self_collide != false {
            os.write_bool(4, self.self_collide)?;
        }
        if self.gravity != false {
            os.write_bool(5, self.gravity)?;
        }
        if self.kinematic != false {
            os.write_bool(6, self.kinematic)?;
        }
        if self.enabled != false {
            os.write_bool(7, self.enabled)?;
        }
        if let Some(v) = self.density.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.inertial.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.visual {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.collision {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.sensor {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for v in &self.projector {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.canonical != false {
            os.write_bool(15, self.canonical)?;
        }
        for v in &self.battery {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        };
        for v in &self.light {
            ::protobuf::rt::write_message_field_with_cached_size(17, v, os)?;
        };
        for v in &self.particle_emitter {
            ::protobuf::rt::write_message_field_with_cached_size(18, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Link {
        Link::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.id = 0;
        self.name.clear();
        self.self_collide = false;
        self.gravity = false;
        self.kinematic = false;
        self.enabled = false;
        self.density.clear();
        self.inertial.clear();
        self.pose.clear();
        self.visual.clear();
        self.collision.clear();
        self.sensor.clear();
        self.projector.clear();
        self.canonical = false;
        self.battery.clear();
        self.light.clear();
        self.particle_emitter.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Link {
        static instance: Link = Link {
            header: ::protobuf::MessageField::none(),
            id: 0,
            name: ::std::string::String::new(),
            self_collide: false,
            gravity: false,
            kinematic: false,
            enabled: false,
            density: ::protobuf::MessageField::none(),
            inertial: ::protobuf::MessageField::none(),
            pose: ::protobuf::MessageField::none(),
            visual: ::std::vec::Vec::new(),
            collision: ::std::vec::Vec::new(),
            sensor: ::std::vec::Vec::new(),
            projector: ::std::vec::Vec::new(),
            canonical: false,
            battery: ::std::vec::Vec::new(),
            light: ::std::vec::Vec::new(),
            particle_emitter: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Link {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Link").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Link {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Link {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12gz/msgs/link.proto\x12\x07gz.msgs\x1a\x16gz/msgs/inertial.proto\
    \x1a\x17gz/msgs/collision.proto\x1a\x14gz/msgs/visual.proto\x1a\x13gz/ms\
    gs/light.proto\x1a\x14gz/msgs/sensor.proto\x1a\x1egz/msgs/particle_emitt\
    er.proto\x1a\x17gz/msgs/projector.proto\x1a\x12gz/msgs/pose.proto\x1a\
    \x15gz/msgs/battery.proto\x1a\x15gz/msgs/density.proto\x1a\x14gz/msgs/he\
    ader.proto\"\xb1\x05\n\x04Link\x12'\n\x06header\x18\x01\x20\x01(\x0b2\
    \x0f.gz.msgs.HeaderR\x06header\x12\x0e\n\x02id\x18\x02\x20\x01(\rR\x02id\
    \x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12!\n\x0cself_collide\
    \x18\x04\x20\x01(\x08R\x0bselfCollide\x12\x18\n\x07gravity\x18\x05\x20\
    \x01(\x08R\x07gravity\x12\x1c\n\tkinematic\x18\x06\x20\x01(\x08R\tkinema\
    tic\x12\x18\n\x07enabled\x18\x07\x20\x01(\x08R\x07enabled\x12*\n\x07dens\
    ity\x18\x08\x20\x01(\x0b2\x10.gz.msgs.DensityR\x07density\x12-\n\x08iner\
    tial\x18\t\x20\x01(\x0b2\x11.gz.msgs.InertialR\x08inertial\x12!\n\x04pos\
    e\x18\n\x20\x01(\x0b2\r.gz.msgs.PoseR\x04pose\x12'\n\x06visual\x18\x0b\
    \x20\x03(\x0b2\x0f.gz.msgs.VisualR\x06visual\x120\n\tcollision\x18\x0c\
    \x20\x03(\x0b2\x12.gz.msgs.CollisionR\tcollision\x12'\n\x06sensor\x18\r\
    \x20\x03(\x0b2\x0f.gz.msgs.SensorR\x06sensor\x120\n\tprojector\x18\x0e\
    \x20\x03(\x0b2\x12.gz.msgs.ProjectorR\tprojector\x12\x1c\n\tcanonical\
    \x18\x0f\x20\x01(\x08R\tcanonical\x12*\n\x07battery\x18\x10\x20\x03(\x0b\
    2\x10.gz.msgs.BatteryR\x07battery\x12$\n\x05light\x18\x11\x20\x03(\x0b2\
    \x0e.gz.msgs.LightR\x05light\x12C\n\x10particle_emitter\x18\x12\x20\x03(\
    \x0b2\x18.gz.msgs.ParticleEmitterR\x0fparticleEmitterB\x19\n\x0bcom.gz.m\
    sgsB\nLinkProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::inertial::file_descriptor().clone());
            deps.push(super::collision::file_descriptor().clone());
            deps.push(super::visual::file_descriptor().clone());
            deps.push(super::light::file_descriptor().clone());
            deps.push(super::sensor::file_descriptor().clone());
            deps.push(super::particle_emitter::file_descriptor().clone());
            deps.push(super::projector::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::battery::file_descriptor().clone());
            deps.push(super::density::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Link::generated_message_descriptor_data());
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
