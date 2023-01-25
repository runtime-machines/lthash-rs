use lthash_rs::LtHash;
use sha3::Shake128;

const LORUM: &str = include_str!("./test-data/lorum-ipsum.txt");
const HASH: &str = include_str!("./test-data/lorum-hash.txt");

type LtHash16 = lthash_rs::LtHash16<Shake128>;

#[test]
fn add_sub_object() {
    let mut lthash = LtHash16::new();
    let elements = ["apple", "banana", "kiwi"];
    lthash.insert(elements[0]);
    lthash.insert(elements[1]);
    lthash.insert(elements[2]);
    lthash.remove(elements[1]);
    let mut lthash_bis = LtHash16::new();
    lthash_bis.insert(elements[0]);
    lthash_bis.insert(elements[2]);
    assert_eq!(lthash.as_bytes(), lthash_bis.as_bytes());
}

#[test]
fn add_with_extend_sub_object() {
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
fn lorum_ipsum_add() {
    let mut lthash = LtHash16::new();
    for object in LORUM.lines() {
        lthash.insert(object);
    }

    assert_eq!(lthash.to_hex_string(), HASH.trim());
}

#[test]
fn lorum_ipsum_add_with_extend() {
    let mut lthash = LtHash16::new();

    lthash.extend(LORUM.lines());

    assert_eq!(lthash.to_hex_string(), HASH.trim());
}
