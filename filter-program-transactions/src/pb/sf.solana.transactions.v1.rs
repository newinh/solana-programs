// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TransactionStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStats {
    #[prost(uint32, required, tag="1")]
    pub block_slot: u32,
    #[prost(string, required, tag="2")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="3")]
    pub index: u32,
    #[prost(uint32, required, tag="4")]
    pub required_signatures: u32,
    #[prost(uint32, required, tag="5")]
    pub readonly_signed_accounts: u32,
    #[prost(uint32, required, tag="6")]
    pub readonly_unsigned_accounts: u32,
    #[prost(string, required, tag="7")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, required, tag="8")]
    pub success: bool,
    #[prost(message, repeated, tag="9")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(uint64, repeated, packed="false", tag="10")]
    pub pre_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, packed="false", tag="11")]
    pub post_balances: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="12")]
    pub pre_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(message, repeated, tag="13")]
    pub post_token_balances: ::prost::alloc::vec::Vec<TokenBalance>,
    #[prost(string, repeated, tag="14")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="15")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, required, tag="16")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="17")]
    pub executing_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, required, tag="18")]
    pub logs_truncated: bool,
    #[prost(string, required, tag="19")]
    pub program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, required, tag="1")]
    pub executing_account: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub account_arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="3")]
    pub data: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub inner_instructions: ::prost::alloc::vec::Vec<InnerInstruction>,
    #[prost(string, repeated, tag="8")]
    pub program_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub program_data: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerInstruction {
    #[prost(string, required, tag="1")]
    pub executing_account: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub account_arguments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, required, tag="3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub program_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub program_data: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalance {
    #[prost(string, required, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(double, required, tag="2")]
    pub amount: f64,
    #[prost(string, required, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub program: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
