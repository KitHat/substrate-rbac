use crate as pallet_rbac;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    storage::bounded_vec::BoundedVec,
    traits::{ConstU16, ConstU64},
};
use scale_info::TypeInfo;
use sp_core::{ConstU32, H256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        RBACModule: pallet_rbac,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

#[derive(Clone, Debug, Encode, Decode, Eq, MaxEncodedLen, PartialEq, TypeInfo)]
pub struct RoleInfo {
    pub name: BoundedVec<u8, ConstU32<20>>,
    pub granters: BoundedVec<u32, ConstU32<20>>,
}

type RoleId = u32;

impl pallet_rbac::RoleInfo<RoleId> for RoleInfo {
    fn new(name: &[u8], granters: &[RoleId]) -> Self {
        RoleInfo {
            name: name.to_vec().try_into().unwrap(),
            granters: granters.to_vec().try_into().unwrap(),
        }
    }

    fn granters(&self) -> &[RoleId] {
        &self.granters
    }

    fn name(&self) -> &[u8] {
        &self.name
    }

    fn add_granter(&mut self, new_granter: RoleId) {
        self.granters.try_push(new_granter).unwrap()
    }
}

impl pallet_rbac::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type RoleId = RoleId;
    type RoleInfo = RoleInfo;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
