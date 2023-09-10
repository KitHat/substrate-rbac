# Role-Based Access Control Pallet Design

## Motivation

The RBAC pallet should provide the functionality of:

* Store the list of roles
* Define the roles
* Grant roles to users
* Revoke roles from users
* Check if user is assigned for the role

This list is not exhaustive, it is a base functionality that pallet.

The naive implementation would be to implement all state changes (role creation, granting and revoking) as extrinsics, but in my opinion it would be unusable. I think that a good RBAC pallet should give an ability to other pallets and use them from their code.

## Interface of pallet

### Entities

* `AccountId` - will be taken from `frame_system::Config`. Basically it is an associated type from system config.
* `RoleId` - id that you can check the role by. It should be to other pallets as they will need to create their own roles.
* `RoleInfo` - a value that consists from name and list of roles that can grant it. It should be represented to other pallets as they will need to create their own roles. 

### Storages

* `Roles` - a list of roles that exist. Can't be changed by extrinsic.
* `Assignments` - a mapping from user to a list of roles that they are assigned to. Can be changed by extrinsic

### Extrinsics

* `assign_role(AccountId, RoleId)` -- adds a role to a user. Must check that caller has needed role to provide this access.
* `revoke_role(AccountId, RoleId)` -- removes a role to a user. Must check that caller has needed role to provide this access.

## Implementation details

### Role definition

Here comes some additional requirements:

* each pallet should be able to define their own roles with their own requirements for granting and revoking
* each pallet should be able to embed their roles into RBAC pallet

It means the we need an additional call for this pallet that *will not be an extrinsic*:

`add_role(RoleId, RoleInfo)` -- adds a role with a specified name and list of roles that can grant it.

Ideally it should be represented through the trait to abstract out pallet from the RBAC implementation and make it replaceable. Trait should be placed in some common crate. 

### Authorization

Pallets will need to check if user is authorized against some role. It is definitely not an extrinsic (as it is not a transaction), so we need to make a getter-ish function for this pallet that will check if user has this role.

* `authorize(AccountId, RoleId) -> bool` -- checks if a user has this role. This is called `authorize` because it is intended to be called from other pallets and for them it is basically an authorization.


### Data Initialization

Other pallets should be able to set up their roles during their initialization.

In Substrate we have two ways of initializing of pallet:

* Genesis config
* Migration

Both of these methods are acceptable, but should be used at a different times. When the developers are starting the chain from scratch, they should use Genesis config and create roles from `BuildGenesisConfig` trait implementation.
When they are introducing new pallets through runtime upgrade, they should write a migration and call it in the `on_runtime_upgrade` hook.

In the examples and tests I will use the `BuildGenesisConfig` option.