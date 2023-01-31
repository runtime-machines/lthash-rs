use lthash_rs::LtHash;
use sha3::Shake128;

const LORUM: &str = include_str!("./test-data/lorum-ipsum.txt");
const HASH: &str = include_str!("./test-data/lorum-hash-16.txt");

type LtHash16 = lthash_rs::LtHash16<Shake128>;

#[test]
fn insert_remove_object() {
    let mut lthash = LtHash16::new();
    let elements = ["apple", "banana", "kiwi"];
    lthash.insert(elements[0]);
    lthash.insert(elements[1]);
    lthash.insert(elements[2]);
    lthash.remove(elements[1]);
    let mut lthash_bis = LtHash16::new();
    lthash_bis.insert(elements[0]);
    lthash_bis.insert(elements[2]);
    assert_eq!(lthash.into_bytes(), lthash_bis.into_bytes());
}

#[test]
fn insert_with_extend_remove_object() {
    let mut lthash = LtHash16::new();

    let objects = vec!["apple", "banana", "kiwi"];
    lthash.extend(&objects);
    lthash.remove(b"banana");

    let mut lthash_bis = LtHash16::new();
    let objects = vec!["apple", "kiwi"];
    lthash_bis.extend(&objects);
    assert_eq!(lthash, lthash_bis, "values don't match");
}

#[test]
fn lorum_ipsum_insert() {
    let mut lthash = LtHash16::new();
    for object in LORUM.lines() {
        lthash.insert(object);
    }

    assert_eq!(lthash.to_hex_string(), HASH.trim());
}

#[test]
fn lorum_ipsum_insert_with_extend() {
    let mut lthash = LtHash16::new();

    lthash.extend(LORUM.lines());

    assert_eq!(lthash.to_hex_string(), HASH.trim());
}

#[test]
fn union() {
    let mut left = LtHash16::new();
    left.insert("hello");

    let mut right = LtHash16::new();
    right.insert("world");

    assert_eq!(left.union(&right), LtHash16::from_iter(["hello", "world"]));
}

#[test]
fn bitor() {
    let mut left = LtHash16::new();
    left.insert("hello");

    let mut right = LtHash16::new();
    right.insert("world");

    assert_eq!(&left | &right, LtHash16::from_iter(["hello", "world"]));
    assert_eq!(left | right, LtHash16::from_iter(["hello", "world"]));
}

#[test]
fn difference() {
    let mut left = LtHash16::new();
    left.extend(["hello", "world"]);

    let mut right = LtHash16::new();
    right.insert("world");

    assert_eq!(left.difference(&right), LtHash16::from_iter(["hello"]));
}

#[test]
fn sub() {
    let mut left = LtHash16::new();
    left.extend(["hello", "world"]);

    let mut right = LtHash16::new();
    right.insert("world");

    assert_eq!(&left - &right, LtHash16::from_iter(["hello"]));
    assert_eq!(left - right, LtHash16::from_iter(["hello"]));
}

#[test]
fn into_from_bytes() {
    let mut left = LtHash16::new();
    left.extend(["hello", "world"]);

    let bytes = left.into_bytes();

    let right = LtHash16::try_from(bytes.as_ref()).unwrap();

    let mut left = LtHash16::new();
    left.extend(["hello", "world"]);

    assert_eq!(left, right);
}

#[test]
fn reset() {
    let mut reset_lthash = LtHash16::new();
    reset_lthash.extend(["hello", "world"]);
    reset_lthash.reset();

    let new_lthash = LtHash16::new();

    assert_eq!(reset_lthash, new_lthash);
}

#[test]
fn invalid_bytes_try_from() {
    let bytes = vec![0u8, 1u8];

    let lthash = LtHash16::try_from(bytes.as_ref());

    assert!(lthash.is_err())
}
