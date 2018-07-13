#![cfg(test)]

#[test]
fn gpg_sign_arg() {
    assert!(crate::gpg_sign_arg("-bsau"));
    assert!(crate::gpg_sign_arg("-s"));
    assert!(crate::gpg_sign_arg("--sign"));
    assert!(!crate::gpg_sign_arg("--status-fd=2"));
}
