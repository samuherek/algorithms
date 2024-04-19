const INPUT: [i32; 10] = [100, 45, 1, 8, 47895, 5, 56, 23, 0, 89];
const OUTPUT: [i32; 10] = [0, 1, 5, 8, 23, 45, 56, 89, 100, 47895];

fn naive() -> Vec<i32> {
    let mut input = Vec::from(INPUT);

    let mut ticks = 0;
    for _ in 0..input.len() {
        for i in 0..input.len() {
            ticks += 1;
            if i > 0 {
            let prev = input[i - 1];
            let next = input[i];

            if next < prev {
                input[i - 1] = next;
                input[i] = prev;
            }
            }
        }
    }

    println!("input length is: {}, and ticks was: {ticks}", input.len());
    input
}

fn naive_short() -> Vec<i32> {
    let mut input = Vec::from(INPUT);

    let mut ticks = 0;
    for i in 1..input.len() {
        let next = input[i];
        let mut j = i - 1;
        while j >= 0 && next < input[j] {
            ticks += 1;
            let prev = input[j];
            input[j] = next;
            input[j + 1] = prev;
            if j != 0 {
                j -= 1;
            }
        }
    }

    println!("input length is: {}, and ticks was: {ticks}", input.len());
    input
}

fn main() {
    naive_short();
}

#[test]
fn test() {
    let v = Vec::from(OUTPUT);
    assert_eq!(v, naive());
    assert_eq!(v, naive_short());
}
