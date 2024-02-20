use crate::prelude::*;
use std::collections::HashSet;

/// Compares two `ConfigObjectMap` instances and returns the names of objects that are exclusive to
/// each map, as well as the names of objects that are common to both maps.
///
/// # Type Parameters
/// * `T` - The type of the objects stored in the maps. Must implement the `ConfigObject` trait.
///
/// # Arguments
/// * `a` - The first `ConfigObjectMap` to compare.
/// * `b` - The second `ConfigObjectMap` to compare.
///
/// # Returns
/// A tuple containing the following:
/// * `a_exclusive` - A `HashSet<String>` containing the names of objects that are in `a` but not `b`.
/// * `b_exclusive` - A `HashSet<String>` containing the names of objects that are in `b` but not `a`.
/// * `common` - A `HashSet<String>` containing the names of objects that are in both `a` and `b`.
///
/// # Example
/// ```rust
/// use opsview::prelude::*;
/// use opsview::config::{Hashtag, compare_config_object_maps};
///
/// let mut a = ConfigObjectMap::<Hashtag>::new();
/// let mut b = ConfigObjectMap::<Hashtag>::new();
///
/// let hashtag1 = Hashtag::minimal("MyHashtag1").unwrap();
/// let hashtag2 = Hashtag::minimal("MyHashtag2").unwrap();
/// let hashtag3 = Hashtag::minimal("MyHashtag3").unwrap();
///
/// a.add(hashtag1.clone());
/// a.add(hashtag2.clone());
///
/// b.add(hashtag2.clone());
/// b.add(hashtag3.clone());
///
/// let (a_exclusive, b_exclusive, common) = compare_config_object_maps(&a, &b);
///
/// assert_eq!(a_exclusive.len(), 1);
/// assert_eq!(b_exclusive.len(), 1);
/// assert_eq!(common.len(), 1);
/// assert!(a_exclusive.is_disjoint(&b_exclusive));
/// assert!(a_exclusive.contains(&hashtag1.name));
/// assert!(b_exclusive.contains(&hashtag3.name));
/// assert!(common.contains(&hashtag2.name));
/// ```
pub fn compare_config_object_maps<T: ConfigObject>(
    a: &ConfigObjectMap<T>,
    b: &ConfigObjectMap<T>,
) -> (
    HashSet<String>, // a_exclusive
    HashSet<String>, // b_exclusive
    HashSet<String>, // common
) {
    let a_keys: HashSet<String> = a.keys().cloned().collect();
    let b_keys: HashSet<String> = b.keys().cloned().collect();

    let a_exclusive = a_keys.difference(&b_keys).cloned().collect();
    let b_exclusive = b_keys.difference(&a_keys).cloned().collect();
    let common = a_keys.intersection(&b_keys).cloned().collect();

    (a_exclusive, b_exclusive, common)
}

/// Compares two `ConfigRefMap` instances and returns the names of objects that are exclusive to
/// each map, as well as the names of objects that are common to both maps.
///
/// # Type Parameters
/// * `T` - The type of the objects stored in the maps. Must implement the `ConfigRef` trait.
///
/// # Arguments
/// * `a` - The first `ConfigRefMap` to compare.
/// * `b` - The second `ConfigRefMap` to compare.
///
/// # Returns
/// A tuple containing the following:
/// * `a_exclusive` - A `HashSet<String>` containing the names of objects that are in `a` but not `b`.
/// * `b_exclusive` - A `HashSet<String>` containing the names of objects that are in `b` but not `a`.
/// * `common` - A `HashSet<String>` containing the names of objects that are in both `a` and `b`.
///
/// # Example
/// ```rust
/// use opsview::prelude::*;
/// use opsview::config::{Hashtag, HashtagRef, compare_config_ref_maps};
///
/// let mut a_objs = ConfigObjectMap::<Hashtag>::new();
/// let mut b_objs = ConfigObjectMap::<Hashtag>::new();
///
/// let hashtag1 = Hashtag::minimal("MyHashtag1").unwrap();
/// let hashtag2 = Hashtag::minimal("MyHashtag2").unwrap();
/// let hashtag3 = Hashtag::minimal("MyHashtag3").unwrap();
///
/// a_objs.add(hashtag1.clone());
/// a_objs.add(hashtag2.clone());
/// let a_refs: ConfigRefMap<HashtagRef> = ref_map_from(&a_objs);
///
/// b_objs.add(hashtag2.clone());
/// b_objs.add(hashtag3.clone());
/// let b_refs: ConfigRefMap<HashtagRef> = ref_map_from(&b_objs);
///
/// let (a_exclusive, b_exclusive, common) = compare_config_ref_maps(&a_refs, &b_refs);
///
/// assert_eq!(a_exclusive.len(), 1);
/// assert_eq!(b_exclusive.len(), 1);
/// assert_eq!(common.len(), 1);
/// assert!(a_exclusive.is_disjoint(&b_exclusive));
/// assert!(a_exclusive.contains(&hashtag1.name));
/// assert!(b_exclusive.contains(&hashtag3.name));
/// assert!(common.contains(&hashtag2.name));
/// ```
pub fn compare_config_ref_maps<T: ConfigRef>(
    a: &ConfigRefMap<T>,
    b: &ConfigRefMap<T>,
) -> (
    HashSet<String>, // a_exclusive
    HashSet<String>, // b_exclusive
    HashSet<String>, // common
) {
    let a_keys: HashSet<String> = a.keys().cloned().collect();
    let b_keys: HashSet<String> = b.keys().cloned().collect();

    let a_exclusive = a_keys.difference(&b_keys).cloned().collect();
    let b_exclusive = b_keys.difference(&a_keys).cloned().collect();
    let common = a_keys.intersection(&b_keys).cloned().collect();

    (a_exclusive, b_exclusive, common)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_config_object_maps() -> Result<(), OpsviewError> {
        use crate::config::Hashtag;
        let mut old = ConfigObjectMap::<Hashtag>::new();
        let mut new = ConfigObjectMap::<Hashtag>::new();

        let hashtag1 = Hashtag::minimal("MyHashtag")?;
        let hashtag2 = Hashtag::minimal("MyHashtag2")?;
        let hashtag3 = Hashtag::minimal("MyHashtag3")?;

        old.add(hashtag1);
        old.add(hashtag2.clone());

        new.add(hashtag2);
        new.add(hashtag3);

        let (a_exclusive, b_exclusive, common) = compare_config_object_maps(&old, &new);

        assert_eq!(a_exclusive.len(), 1);
        assert_eq!(b_exclusive.len(), 1);
        assert_eq!(common.len(), 1);
        assert!(a_exclusive.is_disjoint(&b_exclusive));

        Ok(())
    }
}
