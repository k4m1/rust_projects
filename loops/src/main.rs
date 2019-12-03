fn main() {

    let a = [1,2,3,4,5];

    let mut count = 0;

    while count < 5 {
        println!("this is the value of index {} in the array {}", count, a[count]);
        count += 1
    }
}
