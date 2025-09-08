use merge_sort::*;

fn main() {
    //let list = vec![1, 2, 3];
    //println!("{}", list.len() / 2);

    //let mid = list.len() / 2;

    //let (left, right) = list.split_at(mid);

    //println!("left: {}\nright: {}", left.len(), right.len());

    //let list = vec![1, 2, 3, 4];
    //println!("{}", list.len() / 2);

    //let mid = list.len() / 2;

    //let (left, right) = list.split_at(mid);

    //println!("left: {}\nright: {}", left.len(), right.len());

    let x = vec![4, 5, 6, 8];
    let y = vec![1, 3, 7, 9];

    let z = merge(&x, &y);

    println!("x: {:?}\ny: {:?}", x, y);

    println!("{:?}", z);

    let unsorted = [3, 2, 8, 4, 7, 4, 5, 9, 6];

    let sorted = merge_sort(&unsorted);

    println!("{:?}", sorted);
}
