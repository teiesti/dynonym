//! Shared types (e.g. for IP address and domain name)

/// A salted and cryptographically hashed string
///
/// A `Hash` represents a string that was salted and cryptographically hashed using the bcrypt
/// algorithm. The salt is stored alongside the hash. A `Hash` is well suited to store encrypted
/// passwords.
///
/// Since `Hash` implements `From<&str>`, the preferred method to obtain a `Hash` is to convert a
/// string slice using `Into<Hash>` as shown in the example below.
///
/// A `Hash` can be compared to a given string slice (== verified) with the method [`is`].
///
/// Because of different, randomly chosen salts, two hashes are (almost) never equal, even if
/// obtained from the exact same plain text.
///
/// [`is`]: #method.is
///
/// # Example
///
/// ```
/// use dynonym::types::Hash;
///
/// // Create
/// let h: Hash = "foo".into();
///
/// // Verify
/// assert!( h.is("foo"));
/// assert!(!h.is("bar"));
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Hash(String);

impl Hash {
    /// Verifies whether `self` is a hashed version of the given string slice.
    pub fn is(&self, plain: &str) -> bool {
        use bcrypt::verify;
        verify(plain, &self.0).unwrap()
    }
}

impl<'a> From<&'a str> for Hash {
    fn from(plain: &'a str) -> Self {
        use bcrypt::{hash, DEFAULT_COST};
        let hash = hash(plain, DEFAULT_COST).unwrap();
        Hash(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_eq() {
        let h: Hash = "foo".into();
        assert!(h.is("foo"));
    }

    #[test]
    fn hash_ne() {
        let h: Hash = "foo".into();
        assert!(!h.is("bar"));
    }

    #[test]
    fn hash_salt() {
        let h1: Hash = "foo".into();
        let h2: Hash = "foo".into();
        assert!(h1 != h2);  // different salts!
    }
}
