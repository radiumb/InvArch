#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::Percent;

/// Voting weight of an IPT
#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum OneOrPercent {
    /// Represents 100%
    One,
    /// Represents 0% - 99% inclusive
    ZeroPoint(Percent),
}

/// Entity is parent or child?
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum Parentage<AccountId, IpsId> {
    /// Parent IP (Account Id of itself)
    Parent(AccountId),
    /// Child IP (Id of the immediate parent, Account Id of the topmost parent)
    Child(IpsId, AccountId),
}

/// Normal or replica IPS
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum IpsType<IpsId> {
    /// Normal IPS (original)
    Normal,
    /// IP Replica (Id of the original IP)
    Replica(IpsId),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum BoolOrWasm<Wasm> {
    Bool(bool),
    Wasm(Wasm),
}

/// Core IP Set struct
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub struct CoreInfo<AccountId, CoreMetadataOf, CoreId, Balance> {
    /// IPS parentage
    pub parentage: Parentage<AccountId, CoreId>,
    /// IPS metadata
    pub metadata: CoreMetadataOf,
    /// Specifically, the supply of IPT0 (ownership) tokens.
    pub supply: Balance,

    /// Aye vote percentage required to execute a multisig call.
    ///
    /// Invariant: If set to `One`, 100% of tokens that have non-zero voting weight must approve
    pub execution_threshold: OneOrPercent,
    pub default_asset_weight: OneOrPercent,
    pub default_permission: bool,
}

/// IPF Info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub struct IpfInfo<AccountId, Data, IpfMetadataOf> {
    /// IPF owner
    pub owner: AccountId,
    /// Original IPF author
    pub author: AccountId,
    /// IPF metadata
    pub metadata: IpfMetadataOf,
    /// IPF data
    pub data: Data,
}

// This is a struct in preparation for having more fields in the future.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct SubIptInfo<IptId, SubAssetMetadata> {
    pub id: IptId,
    pub metadata: SubAssetMetadata,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct CallInfo<Data> {
    pub pallet: Data,
    pub function: Data,
}
