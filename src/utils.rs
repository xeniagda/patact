
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::Hash;

/// Merges two `HashMaps`, consuming the second. Gives an Err if both maps contians the same key
pub fn merge<A, B>(map: &mut HashMap<A, B, RandomState>, mut other: HashMap<A, B, RandomState>) -> Result<(), ()>
    where A: Eq + Hash {
    for (key, val) in other.drain() {
        if map.contains_key(&key) {
            return Err(());
        }
        map.insert(key, val);
    }
    Ok(())
}

/// Finds a character not wrappen in delimiters
pub fn find_depth0<F>(input: &str, to_find: F, delim_start: char, delim_end: char) -> Vec<usize>
where
    F: Fn(char) -> bool,
{
    let mut depth = 0u16;
    let mut res = vec![];
    for (i, ch) in input.chars().enumerate() {
        if ch == delim_start {
            depth += 1;
        } else if ch == delim_end {
            if depth == 0 {
                return vec![];
            }
            depth -= 1;
        } else if depth == 0 && to_find(ch) {
            res.push(i);
        }
    }
    res
}

#[test]
fn test_find_depth0() {
    assert_eq!(find_depth0("1+(1+2)+1", |c| c == '+', '(', ')'), vec![1, 7]);
    assert_eq!(
        find_depth0("a!hejbhea!jdbåa!a!hejbcb!", |c| c == '!', 'a', 'b'),
        vec![24]
    );
    assert_eq!(
        find_depth0("!a!hejbh!ejd!åa!a!hejbcb!", |c| c == '!', 'a', 'b'),
        vec![0, 8, 12, 24]
    );
}

