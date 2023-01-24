use sha3::Shake256;
use lthash_rs::lthash::LtHash;

#[test]
fn add_object() {
    let mut lthash = lthash_rs::lthash::LtHash16::<Shake256>::default();
    let elements = ["apple", "banana", "kiwi"];
    lthash.insert(elements[0]);
    lthash.insert(elements[1]);
    lthash.insert(elements[2]);
    lthash.remove(elements[1]);
    let mut lthash_bis = lthash_rs::lthash::LtHash16::<Shake256>::default();
    lthash_bis.insert(elements[0]);
    lthash_bis.insert(elements[2]);
    assert_eq!(lthash.checksum(),lthash_bis.checksum());
}