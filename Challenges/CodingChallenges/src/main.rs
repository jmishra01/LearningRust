
fn median(mut a: Vec<f32>) -> Option<f32> {

    if a.is_empty() {
        return None
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements = a.len();
    let middle = n_elements / 2;

    if n_elements%2 == 1 {
        Some(a[middle])
    } else {
        Some((a[middle] + a[middle - 1]) / 2.0)
    }
}

fn main() {
    println!("Hello, world!");

    let arr: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
    let m = median(arr);

    println!("Median value {}", m.unwrap());
}
