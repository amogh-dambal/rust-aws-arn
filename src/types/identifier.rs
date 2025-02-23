//! Primitives for low-level identifiers that comprise Amazon Resource Names.
use regex::{Captures, Regex};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::{ArnError, ArnResult};

pub(crate) const ARN_PREFIX: &str = "arn";

pub(crate) const PART_SEPARATOR: char = ':';
const PATH_SEPARATOR: char = '/';

const STRING_WILD_ANY: &str = "*";

const CHAR_ASCII_START: char = '\u{1F}';
const CHAR_ASCII_END: char = '\u{7F}';
const CHAR_SPACE: char = ' ';
const CHAR_WILD_ONE: char = '?';
const CHAR_WILD_ANY: char = '*';

pub(crate) const REQUIRED_COMPONENT_COUNT: usize = 6;

static REGEX_VARIABLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\$\{([^$}]+)\}").expect("failed to initialize regex"));

/// This trait is implemented by the `Arn` component types. It
/// represents a string-based identifier that is generally constructed using
/// `FromStr::from_str`.
///
pub trait IdentifierLike
where
    Self: Clone + Display + FromStr + Deref<Target = str>,
{
    /// Construct a new `Identifier` from the provided string **without** checking it's validity.
    /// This can be a useful method to improve performance for statically, or well-known, values;
    /// however, in general `FromStr::from_str` should be used.
    fn new_unchecked(s: &str) -> Self
    where
        Self: Sized;

    /// Returns `true` if the provided string is a valid `Identifier` value, else `false`.
    fn is_valid(s: &str) -> bool;

    /// Construct an account identifier that represents *any*.
    fn any() -> Self {
        Self::new_unchecked(STRING_WILD_ANY)
    }

    /// Return `true` if this is simply the *any* wildcard, else `false`.
    fn is_any(&self) -> bool {
        self.deref().chars().any(|c| c == CHAR_WILD_ANY)
    }

    /// Returns `true` if this identifier contains any wildcard characeters,
    /// else `false`.
    fn has_wildcards(&self) -> bool {
        self.deref()
            .chars()
            .any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
    }

    /// Return `true` if this identifier has no wildcards, else `false`.
    fn is_plain(&self) -> bool {
        !self.has_wildcards()
    }
}

/// A string value that is used to capture the partition, service, and region components
/// of an Arn. These are ASCII only, may not include control characters, spaces, '/', or ':'.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Identifier(String);

///
/// A string value that is used to capture the account ID component
/// of an Arn. These are ASCII digits only and a fixed length of 12 characters.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccountIdentifier(String);

/// A string value that is used to capture the resource component of an Arn. These are ASCII only,
/// may not include control characters but unlike `Identifier` they may include spaces, '/', and ':'.
///
/// > *The content of this part of the Arn varies by service. A resource identifier can be the name
/// > or ID of the resource (for example, `user/Bob` or `instance/i-1234567890abcdef0`) or a
/// > resource path. For example, some resource identifiers include a parent resource
/// > (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a version
/// > (`resource-type:resource-name:qualifier`).*
///
/// > *Some resource Arns can include a path. For example, in Amazon S3, the resource identifier
/// > is an object name that can include slashes ('/') to form a path. Similarly, IAM user names
/// > and group names can include paths.*
///
/// > *In some circumstances, paths can include a wildcard character, namely an asterisk ('*').*
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ResourceIdentifier(String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Identifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidIdentifier(s.to_string()))
        }
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for Identifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.chars().all(|c| {
                c > CHAR_ASCII_START
                    && c < CHAR_ASCII_END
                    && c != CHAR_SPACE
                    && c != PATH_SEPARATOR
                    && c != PART_SEPARATOR
            })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AccountIdentifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidAccountId(s.to_string()))
        }
    }
}

impl From<AccountIdentifier> for String {
    fn from(v: AccountIdentifier) -> Self {
        v.0
    }
}

impl Deref for AccountIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for AccountIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        (s.len() == 12 && s.chars().all(|c| c.is_ascii_digit()))
            || (!s.is_empty()
                && s.len() <= 12
                && s.chars()
                    .all(|c| c.is_ascii_digit() || c == CHAR_WILD_ONE || c == CHAR_WILD_ANY)
                && s.chars().any(|c| c == CHAR_WILD_ONE || c == CHAR_WILD_ANY))
    }
}

impl Display for ResourceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceIdentifier {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ArnError::InvalidResource(s.to_string()))
        }
    }
}

impl From<ResourceIdentifier> for String {
    fn from(v: ResourceIdentifier) -> Self {
        v.0
    }
}

impl From<Identifier> for ResourceIdentifier {
    fn from(v: Identifier) -> Self {
        ResourceIdentifier::new_unchecked(&v.0)
    }
}

impl Deref for ResourceIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierLike for ResourceIdentifier {
    fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_valid(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c > '\u{1F}' && c < '\u{7F}')
    }

    fn is_plain(&self) -> bool {
        !self.has_wildcards() && !self.has_variables()
    }
}

impl ResourceIdentifier {
    /// Construct a resource identifier, as a path, using the `Identifier` path components.
    pub fn from_id_path(path: &[Identifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `Identifier` path components.
    pub fn from_qualified_id(qualified: &[Identifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a path, using the `ResourceIdentifier` path components.
    pub fn from_path(path: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &path
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR.to_string()),
        )
    }

    /// Construct a resource identifier, as a qualified ID, using the `ResourceIdentifier` path components.
    pub fn from_qualified(qualified: &[ResourceIdentifier]) -> Self {
        Self::new_unchecked(
            &qualified
                .iter()
                .map(ResourceIdentifier::to_string)
                .collect::<Vec<String>>()
                .join(&PART_SEPARATOR.to_string()),
        )
    }

    /// Return `true` if this identifier contains path separator characters, else `false`.
    pub fn contains_path(&self) -> bool {
        self.0.contains(PATH_SEPARATOR)
    }

    /// Return the list of path components when split using the path separator character.
    pub fn path_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PATH_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }

    /// Return `true` if this identifier contains qualifier separator characters, else `false`.
    pub fn contains_qualified(&self) -> bool {
        self.0.contains(PART_SEPARATOR)
    }

    /// Return the list of path components when split using the qualifier separator character.
    pub fn qualifier_split(&self) -> Vec<ResourceIdentifier> {
        self.0
            .split(PART_SEPARATOR)
            .map(ResourceIdentifier::new_unchecked)
            .collect()
    }

    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        REGEX_VARIABLE.is_match(self.deref())
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> ArnResult<Self>
    where
        V: Clone + Into<String>,
    {
        let new_text = REGEX_VARIABLE.replace_all(self.deref(), |caps: &Captures<'_>| {
            if let Some(value) = context.get(&caps[1]) {
                value.clone().into()
            } else {
                format!("${{{}}}", &caps[1])
            }
        });
        Self::from_str(&new_text)
    }
}
