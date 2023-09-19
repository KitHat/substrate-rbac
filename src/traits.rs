// The traits below are created to allow loose coupling between RBAC pallets and its consumers.
// Ideally they should be placed in some common crate and imported from it by implementers and by comsu,ers.

/// Trait describing the authorization call
pub trait Authorize<AId, RId> {
    /// Authorize the user against some role list
    ///
    /// **Parameters**:
    /// - `user`: account to check against the roles
    /// - `roles`: role array to check against
    fn authorize(user: &AId, roles: &[RId]) -> bool;
}

/// Trait describing the add role call
pub trait AddRole<Id> {
    /// Add a new role to the role list
    /// This should be called only from `GenesisBuild` or `Hooks::on_runtime_upgrade`
    ///
    /// **Parameters**:
    /// - `name`: slice of bytes representing the role name
    /// - `granters`: slice of ids who can grant the role
    /// - `can_assign_itself`: if set to true, then after id generation it will be added as a granter to role
    ///
    /// **Returns**: generated role id
    fn add_role(
        name: &[u8],
        granters: &[Id],
        can_assign_itself: bool,
    ) -> Result<Id, InterfaceError>;
}

/// Trait describing the preassign role call
pub trait PreassignRole<AId, RId> {
    /// Add a new role holder
    /// This should be called only from `GenesisBuild` or `Hooks::on_runtime_upgrade`
    ///
    /// **Parameters**:
    /// - `user`: user to assign role to
    /// - `role`: role to assign to this user
    ///
    /// **Errors**:
    /// - `RoleNotExist` if there is no role for this `role_id`
    fn preassign_role(user: AId, role: RId) -> Result<(), InterfaceError>;
}

#[derive(Debug, PartialEq)]
pub enum InterfaceError {
    RoleNotExist,
    NameTooLong { expected: u32, observed: usize },
    GrantersListTooLong { expected: u32, observed: usize },
}
