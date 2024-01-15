use std::collections::HashSet;


fn updated(s1: &mut HashSet<String>, s2: &HashSet<String>) {
    s1.extend(s2.iter().map(|x| x.to_owned()));
}


fn test(s1: &mut HashSet<String>, s2: &HashSet<String>) {
    println!("{:#?}", s1);
    updated(s1, s2);
    println!("{:#?}", s1);
}


pub fn run() {
    let mut s1 = HashSet::from(["hello".to_string(), "world".to_string(), "Bhopal".to_string()]);
    let s2 = HashSet::from(["Indore".to_string(), "Jabalpur".to_string(), "Bhopal".to_string()]);

    test(&mut s1, &s2);
}
