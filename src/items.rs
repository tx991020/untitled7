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
