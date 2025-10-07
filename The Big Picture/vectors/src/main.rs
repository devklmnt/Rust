fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let mut values = vec![8, 30]; 
    let mut sum = 0;

    values.push(1);
    values.push(3);

    for n in values {
        sum = add(sum, n);
    }

    println!("sum = {}",sum);
}
