//! Password-encrypted workbooks. Excel wraps the ordinary OOXML package in a compound file with
//! two streams, `EncryptionInfo` and `EncryptedPackage`. `craft-offcrypto` reads them (Agile and
//! Standard encryption; RC4, Extensible, certificate-only and rights-managed files are named as
//! unsupported); this module keeps GridCraft's error type.

/// Why an encrypted workbook couldn't be decrypted.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DecryptError {
    #[error("the password is incorrect")]
    WrongPassword,
    /// The scheme, e.g. "RC4" or "Extensible", to complete "uses {0} encryption".
    #[error("{0} encryption isn't supported")]
    Unsupported(String),
    #[error("the encrypted file is damaged ({0})")]
    Malformed(String),
}

impl From<craft_offcrypto::Error> for DecryptError {
    fn from(e: craft_offcrypto::Error) -> Self {
        use craft_offcrypto::Error as E;
        match e {
            E::WrongPassword | E::PasswordRequired => DecryptError::WrongPassword,
            E::Unsupported(scheme) => DecryptError::Unsupported(scheme),
            E::Malformed(detail) | E::TooLarge(detail) => DecryptError::Malformed(detail),
            e => DecryptError::Malformed(e.to_string()),
        }
    }
}

/// The plain OOXML package (a zip) inside an encrypted workbook.
pub fn decrypt(bytes: &[u8], password: &str) -> Result<Vec<u8>, DecryptError> {
    craft_offcrypto::decrypt(bytes, Some(password)).map_err(DecryptError::from)
}

/// Whether the right password could decrypt this encrypted workbook, checked without one. An
/// error names a scheme no password opens (RC4, rights management…) or damage, so callers can
/// say so instead of asking for a password first.
pub fn check_encryption(bytes: &[u8]) -> Result<(), DecryptError> {
    craft_offcrypto::check(bytes).map_err(DecryptError::from)
}

/// Whether a compound file holds an encryption stream, at any depth. A damaged directory falls
/// back to a search for the stream names, so the file is still reported as encrypted.
pub(crate) fn is_encrypted(bytes: &[u8]) -> bool {
    ["EncryptedPackage", "EncryptionInfo"].iter().any(|n| craft_cfb::has_entry_named(bytes, n, &craft_cfb::Limits::default()))
}

/// Why a workbook couldn't be encrypted (Tinta Office fork: File › Save with a password).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EncryptError {
    #[error("a password can't be empty")]
    EmptyPassword,
    #[error("a password can't be longer than 255 characters")]
    LongPassword,
    #[error("the workbook couldn't be encrypted ({0})")]
    Failed(String),
}

/// `package` (an `.xlsx` zip) as Excel stores it with a password: Agile encryption (AES-256,
/// SHA-512, 100,000 spins) inside a compound file, written by `craft-offcrypto`.
pub fn encrypt(package: &[u8], password: &str) -> Result<Vec<u8>, EncryptError> {
    if password.is_empty() {
        return Err(EncryptError::EmptyPassword);
    }
    if password.chars().count() > craft_offcrypto::MAX_PASSWORD_CHARS {
        return Err(EncryptError::LongPassword);
    }
    craft_offcrypto::encrypt(package, password).map_err(|e| EncryptError::Failed(e.to_string()))
}

#[cfg(test)]
mod tinta_tests {
    use super::*;

    #[test]
    fn a_workbook_saved_with_a_password_opens_with_it_only() {
        let package = b"PK\x03\x04 not really a zip, but bytes all the same".to_vec();
        let encrypted = encrypt(&package, "Contraseña").unwrap();
        assert!(is_encrypted(&encrypted));
        assert_eq!(check_encryption(&encrypted), Ok(()));
        assert_eq!(decrypt(&encrypted, "Contraseña").unwrap(), package);
        assert_eq!(decrypt(&encrypted, "contraseña"), Err(DecryptError::WrongPassword));
        assert_eq!(encrypt(&package, ""), Err(EncryptError::EmptyPassword));
        assert_eq!(encrypt(&package, &"x".repeat(256)), Err(EncryptError::LongPassword));
    }
}
