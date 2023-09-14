use crate::{
    mock::{new_test_ext, RBACModule, RoleInfo as Role, RuntimeOrigin, System, Test},
    AddRole, Authorize, Error, Event, PreassignRole, RoleInfo,
};
use frame_support::{assert_noop, assert_ok};

/// Add two roles and check that their ids are different
#[test]
fn test_role_add() {
    new_test_ext().execute_with(|| {
        let role_1 = Role::new("role_1".as_bytes(), &[]);

        let role_id_1 = RBACModule::add_role(role_1, true);

        let role_2 = Role::new("role_2".as_bytes(), &[role_id_1]);

        let role_id_2 = RBACModule::add_role(role_2, false);

        assert!(role_id_1 != role_id_2);
    });
}

/// Add a role, grant it to the user, check that user can authorize
/// revoke it from the user and check the authorization once again
#[test]
fn test_grant_revoke_and_authorize() {
    new_test_ext().execute_with(|| {
        // Add admin role
        let role_admin = Role::new("admin".as_bytes(), &[]);

        let role_id_admin = RBACModule::add_role(role_admin, true);

        let account_id_admin = 1_u64;

        RBACModule::preassign_role(account_id_admin, role_id_admin).unwrap();

        // Add user role
        let role_user = Role::new("user".as_bytes(), &[role_id_admin]);

        let role_id_user = RBACModule::add_role(role_user, true);

        // Set the block number so event get written to the chain
        System::set_block_number(1);

        // Grant a role
        let account_id_user = 2_u64;

        assert_ok!(RBACModule::grant_role(
            RuntimeOrigin::signed(account_id_admin),
            account_id_user,
            role_id_user,
        ));

        // Check that the role is granted and user can authorize
        System::assert_last_event(
            Event::RoleGranted {
                user: account_id_user,
                role_id: role_id_user,
            }
            .into(),
        );
        assert!(RBACModule::authorize(&account_id_user, &[role_id_user]));

        // Revoke the role
        assert_ok!(RBACModule::revoke_role(
            RuntimeOrigin::signed(account_id_admin),
            account_id_user,
            role_id_user,
        ));

        // Check that the role is revoked and user can't authorize
        System::assert_last_event(
            Event::RoleRevoked {
                user: account_id_user,
                role_id: role_id_user,
            }
            .into(),
        );
        assert!(!RBACModule::authorize(&account_id_user, &[role_id_user]));
    });
}

// Add a role and try to grant it from the user who is not authorized
#[test]
fn test_grant_not_authorized() {
    new_test_ext().execute_with(|| {
        // Create a role
        let role_admin = Role::new("admin".as_bytes(), &[]);

        let role_id_admin = RBACModule::add_role(role_admin, true);

        // This user is not authorized to grant this role
        let account_id_not_authorized = 2_u64;

        let account_id_user = 3_u64;

        // Set the block number so event get written to the chain
        System::set_block_number(1);

        // Check that not authorized user can't grant this role
        assert_noop!(
            RBACModule::grant_role(
                RuntimeOrigin::signed(account_id_not_authorized),
                account_id_user,
                role_id_admin,
            ),
            Error::<Test>::NotAuthorized
        );

        // Check that user who was tried to grant a role can't authorize
        assert!(!RBACModule::authorize(&account_id_user, &[role_id_admin]));
    });
}

#[test]
fn test_grant_role_not_exists() {
    new_test_ext().execute_with(|| {
        // Set the block number so event get written to the chain
        System::set_block_number(1);

        let account_id_user = 1_u64;
        // there is no role with this id
        let non_existent_role = 1_u32;

        // check that this role is not present
        assert_noop!(
            RBACModule::grant_role(
                RuntimeOrigin::signed(account_id_user),
                account_id_user,
                non_existent_role,
            ),
            Error::<Test>::RoleNotExist
        );
    });
}
