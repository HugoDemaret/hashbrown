#![cfg(not(miri))] // FIXME: takes too long

use hashbrown::HashSet;

#[test]
fn test_hashset_insert_remove() {
    let mut m: HashSet<u64> = HashSet::new();
    
    for i in 0..32 {
        println!("Set :  {m:?}");
        m.insert(i.clone());
        println!("Capacity : {}", m.capacity());
    }

    print!("Set : {m:?}");
    println!("Capacity : {}", m.capacity());
}
