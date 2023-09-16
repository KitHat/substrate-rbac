# Role-Based Access Control Pallet Design

## Motivation

### Possible approaches

We have three ways how we can implement an RBAC pallet:

* Simple role system (roles are not nested in terms of grant and revokation)
    * Pros:
        * really fast to implement
        * easy to use and understand
        * RBAC pallet becomes more agnostic to other pallets and provide general functionality
    * Cons
        * problems with role assignment -- we would have to have some restriction for each role which account can grant this role. It does look like a hack (we are building role functionality and do not use it ourselves)
        * such approach is error-prone (e.g. you can lose keys for role granters, assign a wrong user for role grant)
* Complex role system (roles are nested in terms of grant and revokation):
    * Pros:
        * allows to make intuitive role systems (role hierarchy is built smoothly with such approach)
        * approach is less error-prone (it is easier for users to manage roles than accounts)
    * Cons:
        * takes more time to implement than the first approach, requires more complex types for roles
* Layered role systems (permissions are used to restrict access to functionality, roles are basically a set of permissions)
    * Pros:
        * the most flexible approach -- it makes other pallets agnostic to roles and makes them use permissions to restrict the functionality, that looks more intuitive for developers
        * role becomes a set of permissions that explains what it can do and makes easy to check what it can't do
    * Cons:
        * it will take considerably more time to implement
        * for some users such role system may be overcomplicated (e.g. a number of users would avoid using KeyCloak for their role management because of such approach)

For the work task purposes I chose the second option, because it is a balance of better desicions and time consumption. Also it makes the best fit for the task stated in the document. However, for the real project I would push for the third option and would implement a permission-based system -- from my point of view it is a good idea to divide permissions and roles.

### Functionality breakdown

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
* `RoleInfo` - a value that consists from name and list of roles that can grant it. 

### Storages

* `Roles` - a list of roles that exist. Can't be changed by extrinsic.
* `Assignments` - a mapping from user to a list of roles that they are assigned to. Can be changed by extrinsic
* `IdGenerator` - a storage for the next id.

### Extrinsics

* `assign_role(AccountId, RoleId)` -- adds a role to a user. Must check that caller has needed role to provide this access.
* `revoke_role(AccountId, RoleId)` -- removes a role to a user. Must check that caller has needed role to provide this access.

### Events

* `RoleGranted(AccountId, RoleId)` -- role is granted to the user
* `RoleRevoked(AccountId, RoleId)` -- role is revoked from the user
* `RoleCreated(AccountId, RoleInfo)` -- role is revoked from the user

### Errors

* `NotAuthorized` -- user is not authorized for this action

## Implementation details

### Role definition

Here comes some additional requirements:

* each pallet should be able to define their own roles with their own requirements for granting and revoking
* each pallet should be able to embed their roles into RBAC pallet

It means the we need an additional call for this pallet that *will not be an extrinsic*:

`add_role(name: &[u8], granters: &[RoleId], can_assign_itself: bool) -> Result<RoleId, Error>` -- adds a role with a specified name and list of roles that can grant it.

Ideally it should be represented through the trait to abstract out pallet from the RBAC implementation and make it replaceable. Trait should be placed in some common crate. 

### Authorization

Pallets will need to check if user is authorized against some role. It is definitely not an extrinsic (as it is not a transaction), so we need to make a getter-ish function for this pallet that will check if user has this role.

* `authorize(AccountId, &[RoleId]) -> bool` -- checks if a user has the role from the list of roles. It will allow pallet to check user for the list of roles who are allowed for some action. This is called `authorize` because it is intended to be called from other pallets and for them it is basically an authorization.


### Data Initialization

Other pallets should be able to set up their roles during their initialization.

In Substrate we have two ways of initializing of pallet:

* Genesis config
* Migration

Both of these methods are acceptable, but should be used at a different times. When the developers are starting the chain from scratch, they should use Genesis config and create roles from `BuildGenesisConfig` trait implementation.
When they are introducing new pallets through runtime upgrade, they should write a migration and call it in the `on_runtime_upgrade` hook.

In the examples and tests I will use the `BuildGenesisConfig` option.