use std::rc::Rc;
use std::{collections::HashSet, cell::RefCell};


#[derive(Debug, PartialEq, Eq, Hash)]
struct Name {
    value: String
}

type Rrhs = Rc<RefCell<HashSet<Rc<Name>>>>;

fn update(s1: Rrhs, s2: Rrhs) {
    let x = s2.borrow();
    s1.borrow_mut().extend(x.iter().map(|x| x.to_owned()));
}


fn test(s1: Rrhs, s2: Rrhs) {
    update(s1, s2);
}

pub fn run() {
    let name1 = Name {value: "Bhopal".to_string()};
    let name2 = Name {value: "Indore".to_string()};
    let name3 = Name {value: "Jabalpur".to_string()};

    let mut hs1 = HashSet::new();
    let mut hs2 = HashSet::new();

    hs1.insert(Rc::from(name1));
    hs2.insert(Rc::from(name2));
    hs2.insert(Rc::from(name3));

    let refhs1: Rrhs = Rc::from(RefCell::from(hs1));
    let refhs2: Rrhs = Rc::from(RefCell::from(hs2));

    test(refhs1.clone(), refhs2);

    println!("{:#?}", refhs1);
}
