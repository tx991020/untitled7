/// A snazzy new shirt!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shirt {
    #[prost(string, tag="1")]
    pub color: ::prost::alloc::string::String,
    #[prost(enumeration="shirt::Size", tag="2")]
    pub size: i32,
}
/// Nested message and enum types in `Shirt`.
pub mod shirt {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Size {
        Small = 0,
        Medium = 1,
        Large = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientRequest {
    #[prost(uint32, tag="1")]
    pub num_resp: u32,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<Payload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(map="string, string", tag="1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerResponse {
    #[prost(uint32, tag="1")]
    pub resp_idx: u32,
    /// The number of requests the client sent us
    #[prost(uint32, tag="2")]
    pub num_reqs: u32,
}
