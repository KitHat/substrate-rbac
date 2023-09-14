// The traits below are created to allow loose coupling between RBAC pallets and its consumers.
// Ideally they should be placed in some common crate and imported from it by implementers and by comsu,ers.

/// Trait describing the role
pub trait RoleInfo<Id> {
    /// Create a new instance of RoleInfo
    ///
    /// **Parameters**:
    /// - `name`: slice of bytes representing the name
    /// - `granters`: slice of role ids
    ///
    /// **Returns**: a new instance of RoleInfo
    fn new(name: &[u8], granters: &[Id]) -> Self;
    /// Role name
    ///
    /// **Returns**: slice of bytes representing the name
    fn name(&self) -> &[u8];
    /// Roles who can grant the role
    ///
    /// **Returns**: slice of role ids
    fn granters(&self) -> &[Id];
    /// Adds a granter to the role (used in case of self-granting role)
    ///
    /// **Parameters**:
    /// - `new_granter`: id of role that can grant it   
    fn add_granter(&mut self, new_granter: Id);
}

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
pub trait AddRole<Info, Id> {
    /// Add a new role to the role list
    /// This should be called only from `GenesisBuild` or `Hooks::on_runtime_upgrade`
    ///
    /// **Parameters**:
    /// - `role`: role information
    /// - `can_assign_itself`: if set to true, then after id generation it will be added as a granter to role
    ///
    /// **Returns**: generated role id
    fn add_role(role: Info, can_assign_itself: bool) -> Id;
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

#[derive(Debug)]
pub enum InterfaceError {
    RoleNotExist,
}
