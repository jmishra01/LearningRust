use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct TestHashmap {
    pub test_hashmap: HashMap<String, HashSet<Name>>
}


#[derive(Debug, Hash, PartialEq, Eq)]
struct Name {
    name: String
}

impl Name {
    fn new(name: String) -> Self {
        Name {name}
    }
}

fn insert_new(mp: &mut HashMap<String, HashSet<Name>>) {
    let b = Name::new("Bhopal".to_string());

    let x = mp.get_mut("world").unwrap();
    x.insert(b);
}


fn test(test_hash_map: &mut TestHashmap) {
    let mut mp = &mut test_hash_map.test_hashmap;
    insert_new(&mut mp);
}

fn main() {
    let name = Name::new("hello".to_string());
    let mp: HashMap<String, HashSet<Name>> = HashMap::from([("world".to_string(), HashSet::from([name]))]);

    let mut test_hash_map = TestHashmap {test_hashmap: mp};

    dbg!(&test_hash_map);
    test(&mut test_hash_map);
    dbg!(test_hash_map);
}

