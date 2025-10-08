

fn main() {
    let add = |n1,n2| n1+n2;
    let sum_range = |from, to| {
        let mut sum = 0;
        for n in from..to {
            sum = add(sum, n);
        }
        sum
    };

    println!("sum = {}",sum_range(3,10));
}
