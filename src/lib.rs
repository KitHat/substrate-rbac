//! # Role-based Access Control Pallet
//!
//! ## Overview
//!  
//! The Role-based Access Control allows other pallets to build their role systems and enforce restrictions over their generics.
//!
//! ## Interface
//!
//! ### Dispatchable functions
//!
//! * `grant_role` - grants a role to the user
//! * `revoke_role` - revokes a role from the user
//!
//! ### Public functions
//!
//! * `add_role` - creates a new role
//! * `authorize` - challenges a user against the list of roles
//! * `preassign_role` - assign user to the role prior to any block
//!
//! ## Usage
//!
//! ### Prerequisites
//!
//! Import the traits from pallet to your pallet, create a type in config to initiate it from the runtime crate.
//!
//! ### Initialize roles
//!
//! You should initialize your roles from `on_runtime_upgrade` hook (if your chain is already running) or from `BuildGenesisConfig` trait implementation. Don't forget to save the `RoleId` that is returned from `add_role` call to use it later for challenges. Add some accounts to start giving out roles through `preassign_role` call.
//!
//! ### Challenging against the role
//!
//! To challenge user against the role list you should use `authorize` public call. It will return a boolean value as the status of authorization.
//!
//! ### Code sample
//! ```no_run
//! #[frame_support::pallet]
//! pub mod pallet {
//! 	use super::*;
//!     use pallet_rbac as rbac;
//! 	use frame_support::pallet_prelude::*;
//! 	use frame_system::pallet_prelude::*;
//!     use codec::EncodeLike;
//!
//! 	#[pallet::pallet]
//! 	pub struct Pallet<T>(_);
//!
//!     #[pallet::storage]
//!     pub type NeededRole<T: Config> = StorageValue<_, T::RoleId, ValueQuery>;
//!
//! 	#[pallet::config]
//! 	pub trait Config: frame_system::Config {
//!         // other fields from config
//!         type RoleId: MaxEncodedLen + Decode + EncodeLike + TypeInfo + Default + Clone;
//!         type RBAC: rbac::Authorize<Self::AccountId, Self::RoleId>
//! 			+ rbac::AddRole<Self::RoleId>
//! 			+ rbac::PreassignRole<Self::AccountId, Self::RoleId>;
//!         type AdminAccount: Get<Self::AccountId>;
//!     }
//!
//!  	#[pallet::call]
//! 	impl<T: Config> Pallet<T> {
//! 		#[pallet::weight(0)]
//! 		pub fn authorized_call(origin: OriginFor<T>) -> DispatchResult {
//! 			let sender = ensure_signed(origin)?;
//! 			if !<T::RBAC as rbac::Authorize<T::AccountId, T::RoleId>>::authorize(
//! 				&sender,
//! 				&[NeededRole::<T>::get()],
//! 			) {
//! 				// not authorized
//! 			}
//!             // authorized
//! 			Ok(())
//! 		}
//! 	}
//!
//!     #[pallet::hooks]
//! 	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
//!         fn on_runtime_upgrade() -> Weight {
//! 			let role_id =
//! 				<T::RBAC as rbac::AddRole<T::RoleId>>::add_role("admin".as_bytes(), &[], true)
//! 			        .expect("incorrect role created");
//! 			NeededRole::<T>::set(role_id.clone());
//! 			<T::RBAC as rbac::PreassignRole<T::AccountId, T::RoleId>>::preassign_role(
//! 				T::AdminAccount::get(),
//! 				role_id,
//! 			)
//! 			.expect("there is some problem with setup");
//! 		    Weight::zero()
//! 	    }
//!     }
//! }  
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod traits;
pub use traits::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use core::fmt::Debug;

    use super::*;
    use codec::{Decode, EncodeLike, MaxEncodedLen};
    use frame_support::{
        pallet_prelude::{StorageDoubleMap, ValueQuery, *},
        traits::Incrementable,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;

    #[derive(Clone, Debug, Encode, Decode, MaxEncodedLen, PartialEqNoBound, TypeInfo)]
    #[scale_info(skip_type_params(LN, LG))]
    pub struct RoleInfo<T: TypeInfo + Debug + PartialEq, LN: Get<u32>, LG: Get<u32>> {
        pub name: BoundedVec<u8, LN>,
        pub granters: BoundedVec<T, LG>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Storage for account-role relationship
    #[pallet::storage]
    #[pallet::getter(fn assignments)]
    pub type Assignments<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::RoleId,
        bool,
        ValueQuery,
    >;

    /// Storage for role information
    #[pallet::storage]
    #[pallet::getter(fn roles)]
    pub type Roles<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::RoleId,
        RoleInfo<T::RoleId, T::NameMaxLength, T::GrantersListMaxLength>,
    >;

    /// Storage with the latest role id. Used for ensure that there won't be collisions with role generation.
    #[pallet::storage]
    type IdGenerator<T: Config> = StorageValue<_, T::RoleId, ValueQuery>;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;
        /// Type used for role identification
        type RoleId: Clone
            + Copy
            + Debug
            + Default
            + Decode
            + EncodeLike
            + Eq
            + MaxEncodedLen
            + TypeInfo
            + Incrementable;
        /// Maximum length of role name
        type NameMaxLength: Get<u32> + Clone + Debug;
        /// Maximum length of granters list
        type GrantersListMaxLength: Get<u32> + Clone + Debug;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Role was created
        RoleCreated {
            id: T::RoleId,
            info: RoleInfo<T::RoleId, T::NameMaxLength, T::GrantersListMaxLength>,
        },
        /// Role was granted to the user
        RoleGranted {
            user: T::AccountId,
            role_id: T::RoleId,
        },
        /// Role was revoked from the user
        RoleRevoked {
            user: T::AccountId,
            role_id: T::RoleId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// User is not authorized for such action
        NotAuthorized,
        /// No such role exists
        RoleNotExist,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Grant a role to the user
        ///
        /// Parameters:
        /// - `origin`: role granter.
        /// - `user`: role grantee.
        /// - `role_id`: id of role to grant.
        ///
        /// Events:
        /// - `RoleGranted(user, role_id)` if role is granted
        ///
        /// Errors:
        /// - `NotAuthorized` if `origin` is not authorized to grant this role
        /// - `RoleNotExist`  if there is no role for this `role_id`
        ///
        /// Complexity:
        ///  - O(1)
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::grant_role())]
        pub fn grant_role(
            origin: OriginFor<T>,
            user: T::AccountId,
            role_id: T::RoleId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let role = Roles::<T>::get(role_id);

            let Some(role) = role else {
                Err(Error::<T>::RoleNotExist)?
            };

            if !Pallet::<T>::authorize(&who, role.granters.as_slice()) {
                Err(Error::<T>::NotAuthorized)?
            }

            Assignments::<T>::set(user.clone(), role_id, true);

            Self::deposit_event(Event::RoleGranted { user, role_id });
            Ok(())
        }

        /// Revoke a role from the user
        ///
        /// Parameters:
        /// - `origin`: role revoker.
        /// - `user`: account to revoke a role from.
        /// - `role_id`: id of role to revoke.
        ///
        /// Events:
        /// - `RoleRevoked(user, role_id)` if role is revoked
        ///
        /// Errors:
        /// - `NotAuthorized` if `origin` is not authorized to revoke this role
        /// - `RoleNotExist`  if there is no role for this `role_id`
        ///
        /// Complexity:
        ///  - O(1)
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::revoke_role())]
        pub fn revoke_role(
            origin: OriginFor<T>,
            user: T::AccountId,
            role_id: T::RoleId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let role = Roles::<T>::get(role_id);

            let Some(role) = role else {
                Err(Error::<T>::RoleNotExist)?
            };

            if !Pallet::<T>::authorize(&who, role.granters.as_slice()) {
                Err(Error::<T>::NotAuthorized)?
            }

            Assignments::<T>::remove(user.clone(), role_id);

            Self::deposit_event(Event::RoleRevoked { user, role_id });
            Ok(())
        }
    }

    impl<T: Config> Authorize<T::AccountId, T::RoleId> for Pallet<T> {
        fn authorize(user: &T::AccountId, roles: &[T::RoleId]) -> bool {
            for role in roles {
                let authorized = Assignments::<T>::get(user, role);
                if authorized {
                    return true;
                }
            }
            false
        }
    }

    impl<T: Config> AddRole<T::RoleId> for Pallet<T> {
        fn add_role(
            name: &[u8],
            granters: &[T::RoleId],
            can_assign_itself: bool,
        ) -> Result<T::RoleId, InterfaceError> {
            let next_id = IdGenerator::<T>::mutate(|id| {
                *id = id.increment();
                *id
            });
            let granters = if can_assign_itself {
                [granters, &[next_id]].concat()
            } else {
                granters.to_vec()
            };
            let role = RoleInfo {
                name: name
                    .to_vec()
                    .try_into()
                    .map_err(|_| InterfaceError::NameTooLong {
                        expected: T::NameMaxLength::get(),
                        observed: name.len(),
                    })?,
                granters: granters.clone().try_into().map_err(|_| {
                    InterfaceError::GrantersListTooLong {
                        expected: T::GrantersListMaxLength::get(),
                        observed: granters.len(),
                    }
                })?,
            };
            Roles::<T>::set(next_id, Some(role.clone()));
            Self::deposit_event(Event::RoleCreated {
                id: next_id,
                info: role,
            });
            Ok(next_id)
        }
    }

    impl<T: Config> PreassignRole<T::AccountId, T::RoleId> for Pallet<T> {
        fn preassign_role(user: T::AccountId, role: T::RoleId) -> Result<(), InterfaceError> {
            if !Roles::<T>::contains_key(role) {
                Err(InterfaceError::RoleNotExist)?
            };

            Assignments::<T>::set(user, role, true);

            Ok(())
        }
    }
}
