use std::{error, fmt, hash::Hash};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Fragment {
    Index(usize),
    Name(String),
    Parent,
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fragment::Index(ref index) => write!(f, "{}", index),
            Fragment::Name(ref name) => write!(f, "{}", name),
            Fragment::Parent => write!(f, "^"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "&str")]
pub struct Path {
    pub fragments: Vec<Fragment>,
    pub is_relative: bool,
}

#[derive(Debug)]
pub struct PathError(&'static str);

impl Path {
    fn from_fragments(fragments: Vec<Fragment>, is_relative: bool) -> Path {
        Path {
            fragments: fragments,
            is_relative: is_relative,
        }
    }

    pub fn from_str(path: &str) -> Option<Path> {
        if path.is_empty() {
            return None;
        }

        let is_relative = path.starts_with('.');

        // If the path is relative remove the first dot
        let new_path = if is_relative {
            let mut iter = path.chars();
            iter.next();
            iter.as_str()
        } else {
            path
        };

        let fragments: Vec<Fragment> = new_path
            .split('.')
            .map(|token| match token.parse::<usize>() {
                Ok(index) => Fragment::Index(index),
                Err(_) => {
                    if token == "^" {
                        Fragment::Parent
                    } else {
                        Fragment::Name(token.to_string())
                    }
                }
            })
            .collect();

        Some(Path::from_fragments(fragments, is_relative))
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_relative {
            write!(f, ".")?;
        }

        write!(
            f,
            "{}",
            self.fragments
                .iter()
                .map(|fragment| fragment.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl TryFrom<&str> for Path {
    type Error = PathError;

    fn try_from(string: &str) -> Result<Path, PathError> {
        Self::from_str(string).ok_or(PathError("Failed parsing empty path"))
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for PathError {}

#[cfg(test)]
mod tests {
    use crate::path::Path;

    #[test]
    fn path_display_test() {
        let string = "0.g-0.2.$r1";
        let path = Path::from_str(string).unwrap();
        assert_eq!(format!("{}", path), string);
    }

    #[test]
    fn empty_path_string_test() {
        assert!(Path::from_str("").is_none());
        assert_eq!(
            format!("{}", Path::try_from("").unwrap_err()),
            "Failed parsing empty path"
        )
    }
}
