use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_rbac.
pub trait WeightInfo {
    fn grant_role() -> Weight;
    fn revoke_role() -> Weight;
    fn add_role() -> Weight;
    fn authorize() -> Weight;
}

/// Weights for pallet_rbac.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_role() -> Weight {
        // ideally it should be measured in benchmarks
        Weight::from_parts(6_000_000, 0).saturating_add(T::DbWeight::get().writes(3_u64))
    }

    fn authorize() -> Weight {
        // I assume that in general we will check the user against the list of 2 roles
        Weight::from_parts(6_000_000, 0).saturating_add(T::DbWeight::get().reads(2_u64))
    }

    fn grant_role() -> Weight {
        Weight::from_parts(6_000_000, 0)
            .saturating_add(Self::authorize())
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().reads(2_u64))
    }

    fn revoke_role() -> Weight {
        Weight::from_parts(6_000_000, 0)
            .saturating_add(Self::authorize())
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().reads(2_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: TemplateModule Something (r:0 w:1)
    /// Proof: TemplateModule Something (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    fn add_role() -> Weight {
        // ideally it should be measured in benchmarks
        Weight::from_parts(6_000_000, 0).saturating_add(RocksDbWeight::get().writes(3_u64))
    }

    fn authorize() -> Weight {
        // I assume that in general we will check the user against the list of 2 roles
        Weight::from_parts(6_000_000, 0).saturating_add(RocksDbWeight::get().reads(2_u64))
    }

    fn grant_role() -> Weight {
        Weight::from_parts(6_000_000, 0)
            .saturating_add(Self::authorize())
            .saturating_add(RocksDbWeight::get().writes(2_u64))
            .saturating_add(RocksDbWeight::get().reads(2_u64))
    }

    fn revoke_role() -> Weight {
        Weight::from_parts(6_000_000, 0)
            .saturating_add(Self::authorize())
            .saturating_add(RocksDbWeight::get().writes(2_u64))
            .saturating_add(RocksDbWeight::get().reads(2_u64))
    }
}
