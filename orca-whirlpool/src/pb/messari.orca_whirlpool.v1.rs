// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrcaSwaps {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Swap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(uint64, tag="1")]
    pub amount: u64,
    #[prost(uint64, tag="2")]
    pub other_amount_threshold: u64,
    #[prost(string, tag="3")]
    pub sqrt_price_limit: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub amount_specified_is_input: bool,
    #[prost(bool, tag="5")]
    pub a_to_b: bool,
    #[prost(string, tag="6")]
    pub signature: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
