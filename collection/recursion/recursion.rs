use read_input::{self, shortcut::input};

fn permutate(a: Vec<u32>) -> Vec<Vec<u32>> {
    if a.len() == 0 {return vec![vec![0]]}
    let mut answer: Vec<Vec<u32>> = vec![];
    if a.len() == 1 {
        answer.push(a);
    } else if a.len() == 2{
        answer.push(vec![a[0], a[1]]);
        answer.push(vec![a[1], a[0]]);
    } else {
        for (i, &number) in a.clone().iter().enumerate() {
            let mut b = a.clone();
            b.swap(0, i);
            for mut permutation in permutate(b[1..].into()) {
                permutation.insert(0, number);
                answer.push(permutation);
            }
        }
    }
    answer
}

fn main() {
    let n = input::<u32>().get();
    for permutation in permutate((1..n+1).collect::<Vec<u32>>()) {
        println!("{:?}", permutation);
    }
}

/*
012
021
102
120
201
210
*/
