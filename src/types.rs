//! Shared types (e.g. for IP address and domain name)

use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use trust_dns::error::ParseError;
use trust_dns::rr::domain::Name;

/// A domain name
///
/// A `Domain` represents a domain name as used in the Domain Name System (DNS).
///
/// Warning: At the moment, a `Domain` is merely a tuple struct wrapping a `String`. The API even
/// provides public access to the inner field. However, this is not considered to be stable and
/// may change in the future without being considered to be a breaking change.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Domain(pub String);

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Domain {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Domain(s.into()))
    }
}

impl<'v> FromFormValue<'v> for Domain {
    type Error = &'v RawStr;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let inner = String::from_form_value(v)?;
        Ok(Domain(inner))
    }
}

impl TryInto<Name> for Domain {
    type Error = ParseError;
    fn try_into(self) -> Result<Name, Self::Error> {
        Name::parse(&self.0, None)
    }
}

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
